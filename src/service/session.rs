use crate::router::AppState;
use crate::encrypt::Bcrypt;
use serde::Deserialize;
use serde_json::json;
use actix_web::{web, HttpRequest, HttpResponse};
use crate::middleware::{get_client_ip, bearer_auth};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(serde::Serialize)]
pub struct SessionInfo {
    pub id: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub last_accessed_at: chrono::DateTime<chrono::Utc>,
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

pub async fn login_handler(
    state: web::Data<AppState>,
    body: web::Json<LoginRequest>,
    req: HttpRequest,
) -> HttpResponse {
    let user = match state.mysql.get_user_by_email(&body.email).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return auth_error!("Invalid email or password");
        }
        Err(e) => {
            return db_error!("DB_ERROR", "Failed to query user", e);
        }
    };

    let password_valid = match Bcrypt::verify(&body.password, &user.password_hash) {
        Ok(valid) => valid,
        Err(_) => return auth_error!("Invalid email or password"),
    };

    if !password_valid {
        return auth_error!("Invalid email or password");
    }

    let ip_address = get_client_ip(&req);
    let session_key = format!("{}{}{}", Uuid::new_v4(), state.snowflake.generate().await, Uuid::new_v4()).replace("-", "");
    let session_id = state.snowflake.generate().await.to_string();
    let user_agent = req
        .headers()
        .get("User-Agent")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    match state.mysql.create_session(&session_id, &user.id, &session_key, Some(&ip_address), user_agent.as_deref()).await {
        Ok(_) => {
            let sessions = match state.mysql.get_user_sessions(&user.id).await {
                Ok(sessions) => sessions,
                Err(e) => {
                    return auth_error!(e.to_string());
                }
            };

            let filtered_sessions: Vec<SessionInfo> = sessions
                .into_iter()
                .map(|s| SessionInfo {
                    id: s.id,
                    ip_address: s.ip_address,
                    user_agent: s.user_agent,
                    last_accessed_at: s.last_accessed_at,
                })
                .collect();
            
            HttpResponse::Ok().json(json!({
                "data": {
                    "id": user.id,
                    "email": user.email,
                    "icon_url": user.icon_url,
                    "created_at": user.created_at,
                    "session_key": session_key,
                    "session_id": session_id,
                    "sessions": filtered_sessions
                }
            }))
        },
        Err(e) => {
            db_error!("SESSION_CREATE_FAILED", "Failed to create session", e)
        }
    }
}

pub async fn logout_handler(
    state: web::Data<AppState>,
    path: web::Path<String>,
    req: HttpRequest,
) -> HttpResponse {
    let target_session_id = path.into_inner();

    let session = match bearer_auth(state.clone(), &req).await {
        Ok(session) => session,
        Err(e) => return auth_error!(e.to_string()),
    };

    let sessions = match state.mysql.get_user_sessions(&session.user_id).await {
        Ok(sessions) => sessions,
        Err(e) => return db_error!("DB_ERROR", "Failed to get sessions", e),
    };

    let target_session = match sessions.into_iter().find(|s| s.id == target_session_id) {
        Some(s) => s,
        None => {
            return HttpResponse::Forbidden().json(json!({
                "error": { "code": "FORBIDDEN", "message": "Cannot delete other user's session or session not found" }
            }));
        }
    };

    match state.mysql.delete_session(&target_session.session_key).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "data": { "message": "Logged out successfully" }
        })),
        Err(e) => db_error!("SESSION_DELETE_FAILED", "Failed to delete session", e),
    }
}

pub async fn session_info_handler(
    state: web::Data<AppState>,
    req: HttpRequest,
) -> HttpResponse {
    let session = match bearer_auth(state.clone(), &req).await {
        Ok(session) => session,
        Err(e) => {
            return auth_error!(e.to_string());
        }
    };

    let _ = state.mysql.update_session_access_time(&session.session_key).await;

    let sessions = match state.mysql.get_user_sessions(&session.user_id).await {
        Ok(sessions) => sessions,
        Err(e) => {
            return auth_error!(e.to_string());
        }
    };

    let filtered_sessions: Vec<SessionInfo> = sessions
        .into_iter()
        .map(|s| SessionInfo {
            id: s.id,
            ip_address: s.ip_address,
            user_agent: s.user_agent,
            last_accessed_at: s.last_accessed_at,
        })
        .collect();

    match state.mysql.get_user(&session.user_id).await {
        Ok(Some(user)) => HttpResponse::Ok().json(json!({
            "data": {
                "id": user.id,
                "email": user.email,
                "icon_url": user.icon_url,
                "created_at": user.created_at,
                "session_key": session.session_key,
                "session_id": session.id,
                "sessions": filtered_sessions
            }
        })),
        Ok(None) => auth_error!("User not found"),
        Err(e) => db_error!("DB_ERROR", "Failed to retrieve user", e),
    }
}