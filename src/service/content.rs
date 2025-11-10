use actix_web::{web, HttpRequest, HttpResponse};
use crate::router::AppState;
use crate::middleware::bearer_auth;
use serde::Deserialize;
use serde_json::json;
use std::fs;

#[derive(Deserialize)]
pub struct UpdateAccessRequest {
    accessible_user_ids: Vec<String>,
}

#[derive(Deserialize)]
pub struct FileUploadRequest {
    #[serde(default)]
    filename: Option<String>,
}

macro_rules! auth_error {
    ($e:expr) => {
        HttpResponse::Unauthorized().json(json!({
            "error": {
                "code": "AUTH_FAILED",
                "message": "Authentication failed",
                "detail": $e.to_string()
            }
        }))
    };
}

macro_rules! db_error {
    ($code:expr, $msg:expr, $e:expr) => {
        HttpResponse::InternalServerError().json(json!({
            "error": {
                "code": $code,
                "message": $msg,
                "detail": $e.to_string()
            }
        }))
    };
}

pub async fn index_handler() -> HttpResponse {
    match fs::read_to_string("./dist/index.html") {
        Ok(html) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html),
        Err(_) => HttpResponse::InternalServerError()
            .content_type("text/html; charset=utf-8")
            .body("<html><body><h1>500 - Internal Server Error</h1></body></html>"),
    }
}

pub async fn get_turnstile(state: web::Data<AppState>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .body(state.turnstile_sitekey.clone())
}

pub async fn create_file_handler(
    state: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<FileUploadRequest>,
) -> HttpResponse {
    let session = match bearer_auth(state.clone(), &req).await {
        Ok(id) => id,
        Err(e) => return auth_error!(e),
    };

    let file_id = state.snowflake.generate().await.to_string();
    let filename = body.filename.as_deref().unwrap_or(&file_id).to_string();

    if let Err(e) = state.mysql.create_file(&file_id, &filename, &session.user_id, "[]").await {
        return db_error!("DB_ERROR", "Failed to create file record in database", e);
    }

    match state.s3client.upload_url(&file_id).await {
        Ok(url) => HttpResponse::Created().json(json!({
            "data": {
                "id": file_id,
                "filename": filename,
                "url": url,
                "user_id": session.user_id,
                "owner_id": session.user_id
            }
        })),
        Err(e) => db_error!("UPLOAD_FAILED", "Failed to generate upload URL", e),
    }
}

pub async fn update_file_access_handler(
    state: web::Data<AppState>,
    path: web::Path<String>,
    req: HttpRequest,
    body: web::Json<UpdateAccessRequest>,
) -> HttpResponse {
    let session = match bearer_auth(state.clone(), &req).await {
        Ok(id) => id,
        Err(e) => return auth_error!(e),
    };

    let file_id = path.into_inner();

    let is_owner = match state.mysql.check_user_is_owner(&file_id, &session.user_id).await {
        Ok(owner) => owner,
        Err(e) => return db_error!("DB_ERROR", "Failed to verify file ownership", e),
    };

    if !is_owner {
        return HttpResponse::Forbidden().json(json!({
            "error": {
                "code": "ACCESS_DENIED",
                "message": "Only file owner can update access permissions",
                "detail": format!("User {} is not the owner of file {}", session.user_id, file_id)
            }
        }));
    }

    let mut seen = std::collections::HashSet::with_capacity(body.accessible_user_ids.len());
    for uid in &body.accessible_user_ids {
        if !seen.insert(uid.clone()) {
            return HttpResponse::BadRequest().json(json!({
                "error": {
                    "code": "DUPLICATE_USER_ID",
                    "message": "Duplicate user ID in accessible users",
                    "detail": format!("User ID '{}' appears more than once", uid)
                }
            }));
        }
    }

    for user_id_item in &body.accessible_user_ids {
        match state.mysql.get_user(user_id_item).await {
            Ok(Some(_)) => {},
            Ok(None) => {
                return HttpResponse::BadRequest().json(json!({
                    "error": {
                        "code": "USER_NOT_FOUND",
                        "message": "One or more users do not exist",
                        "detail": format!("User with ID '{}' not found", user_id_item)
                    }
                }));
            }
            Err(e) => return db_error!("DB_ERROR", "Failed to verify user existence", e),
        }
    }

    let updated_ids = serde_json::to_string(&body.accessible_user_ids)
        .unwrap_or_else(|_| "[]".to_string());

    match state.mysql.update_file_access(&file_id, &updated_ids).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "data": {
                "message": "File access updated successfully",
                "file_id": file_id,
                "accessible_user_ids": body.accessible_user_ids
            }
        })),
        Err(e) => db_error!("UPDATE_FAILED", "Failed to update file access", e),
    }
}

pub async fn get_file_details_handler(
    state: web::Data<AppState>,
    path: web::Path<String>,
    req: HttpRequest,
) -> HttpResponse {
    let session = match bearer_auth(state.clone(), &req).await {
        Ok(id) => id,
        Err(e) => return auth_error!(e),
    };

    let file_id = path.into_inner();

    let has_access = match state.mysql.check_user_can_access(&file_id, &session.user_id).await {
        Ok(access) => access,
        Err(e) => return db_error!("DB_ERROR", "Failed to check file access", e),
    };

    if !has_access {
        return HttpResponse::Forbidden().json(json!({
            "error": {
                "code": "ACCESS_DENIED",
                "message": "You don't have permission to access this file",
                "detail": format!("User {} cannot access file {}", session.user_id, file_id)
            }
        }));
    }

    match state.mysql.get_file(&file_id).await {
        Ok(Some(file)) => HttpResponse::Ok().json(json!({
            "data": {
                "id": file.id,
                "filename": file.filename,
                "owner_id": file.owner_id,
                "created_at": file.created_at,
                "accessible_user_ids": file.accessible_user_ids
            }
        })),
        Ok(None) => HttpResponse::NotFound().json(json!({
            "error": {
                "code": "FILE_NOT_FOUND",
                "message": "File not found"
            }
        })),
        Err(e) => db_error!("DB_ERROR", "Failed to retrieve file", e),
    }
}

pub async fn get_download_url_handler(
    state: web::Data<AppState>,
    path: web::Path<String>,
    req: HttpRequest,
) -> HttpResponse {
    let session = match bearer_auth(state.clone(), &req).await {
        Ok(id) => id,
        Err(e) => return auth_error!(e),
    };

    let file_id = path.into_inner();

    let has_access = match state.mysql.check_user_can_access(&file_id, &session.user_id).await {
        Ok(access) => access,
        Err(e) => return db_error!("DB_ERROR", "Failed to check file access", e),
    };

    if !has_access {
        return HttpResponse::Forbidden().json(json!({
            "error": {
                "code": "ACCESS_DENIED",
                "message": "You don't have permission to access this file",
                "detail": format!("User {} cannot access file {}", session.user_id, file_id)
            }
        }));
    }

    match state.s3client.download_url(&file_id).await {
        Ok(url) => HttpResponse::Ok().json(json!({
            "data": {
                "url": url,
                "user_id": session.user_id,
                "file_id": file_id
            }
        })),
        Err(e) => db_error!("DOWNLOAD_FAILED", "Failed to generate download URL", e),
    }
}

pub async fn list_files_handler(
    state: web::Data<AppState>,
    req: HttpRequest,
) -> HttpResponse {
    let session = match bearer_auth(state.clone(), &req).await {
        Ok(id) => id,
        Err(e) => return auth_error!(e),
    };

    match state.mysql.list_user_files(&session.user_id).await {
        Ok(files) => HttpResponse::Ok().json(json!({
            "data": {
                "files": files
            }
        })),
        Err(e) => db_error!("LIST_FAILED", "Failed to get file list", e),
    }
}

pub async fn delete_file_handler(
    state: web::Data<AppState>,
    path: web::Path<String>,
    req: HttpRequest,
) -> HttpResponse {
    let session = match bearer_auth(state.clone(), &req).await {
        Ok(id) => id,
        Err(e) => return auth_error!(e),
    };

    let file_id = path.into_inner();

    let is_owner = match state.mysql.check_user_is_owner(&file_id, &session.user_id).await {
        Ok(owner) => owner,
        Err(e) => return db_error!("DB_ERROR", "Failed to check file ownership", e),
    };

    if !is_owner {
        return HttpResponse::Forbidden().json(json!({
            "error": {
                "code": "ACCESS_DENIED",
                "message": "Only file owner can delete this file",
                "detail": format!("User {} is not the owner of file {}", session.user_id, file_id)
            }
        }));
    }

    if let Err(e) = state.s3client.delete_object(&file_id).await {
        return db_error!("DELETE_FAILED", "Failed to delete file from S3", e);
    }

    match state.mysql.delete_file(&file_id).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "data": {
                "message": "File deleted successfully",
                "file_id": file_id
            }
        })),
        Err(e) => db_error!("DB_ERROR", "Failed to delete file from database", e),
    }
}