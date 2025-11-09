use actix_web::{web, HttpRequest, HttpResponse};
use crate::router::AppState;
use crate::encrypt::Bcrypt;
use serde::Deserialize;
use serde_json::json;
use crate::middleware::basic_auth;
use cf_turnstile::SiteVerifyRequest;

const MIN_PASSWORD_LENGTH: usize = 8;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    email: String,
    password: String,
    turnstile: String,
}

struct ValidationError {
    code: &'static str,
    message: &'static str,
    detail: String,
}

impl ValidationError {
    fn to_response(&self) -> HttpResponse {
        HttpResponse::BadRequest().json(json!({
            "error": {
                "code": self.code,
                "message": self.message,
                "detail": self.detail
            }
        }))
    }
}

fn validate_create_user_input(req: &CreateUserRequest) -> Result<(), ValidationError> {
    if req.turnstile.is_empty() {
        return Err(ValidationError {
            code: "INVALID_INPUT",
            message: "Turnstile token is required",
            detail: "Turnstile field must be provided".to_string(),
        });
    }

    if req.email.is_empty() {
        return Err(ValidationError {
            code: "INVALID_INPUT",
            message: "Email is required",
            detail: "Email field must be provided".to_string(),
        });
    }

    if req.password.is_empty() {
        return Err(ValidationError {
            code: "INVALID_INPUT",
            message: "Password is required",
            detail: "Password field must be provided".to_string(),
        });
    }

    if req.password.len() < MIN_PASSWORD_LENGTH {
        return Err(ValidationError {
            code: "WEAK_PASSWORD",
            message: "Password must be at least 8 characters",
            detail: format!(
                "Password length is {}, minimum required is {}",
                req.password.len(),
                MIN_PASSWORD_LENGTH
            ),
        });
    }

    Ok(())
}

macro_rules! error_response {
    ($status:expr, $code:expr, $msg:expr, $detail:expr) => {
        $status.json(json!({
            "error": {
                "code": $code,
                "message": $msg,
                "detail": $detail
            }
        }))
    };
}

pub async fn create_user_handler(
    state: web::Data<AppState>,
    req_body: web::Json<CreateUserRequest>,
) -> HttpResponse {
    if let Err(e) = validate_create_user_input(&req_body) {
        return e.to_response();
    }

    let email = &req_body.email;
    let password = &req_body.password;
    let turnstile = &req_body.turnstile;

    let hashed_password = match Bcrypt::hash(password) {
        Ok(hash) => hash,
        Err(e) => {
            return error_response!(
                HttpResponse::InternalServerError(),
                "HASH_FAILED",
                "Failed to hash password",
                e.to_string()
            )
        }
    };

    let turnstile_request = SiteVerifyRequest {
        response: turnstile.clone(),
        secret: None,
        remote_ip: None,
    };

    match state.turnstile_client.siteverify(turnstile_request).await {
        Ok(response) => {
            if !response.success {
                return error_response!(
                    HttpResponse::BadRequest(),
                    "TURNSTILE_INVALID",
                    "Turnstile verification failed",
                    format!(
                        "hostname: {:?}, timestamp: {:?}, action: {:?}, cdata: {:?}",
                        response.hostname, response.timestamp, response.action, response.cdata
                    )
                );
            }
        }
        Err(e) => {
            return error_response!(
                HttpResponse::InternalServerError(),
                "TURNSTILE_FAILED",
                "Failed to verify Turnstile",
                e.to_string()
            )
        }
    }

    match state.mysql.get_user_by_email(email).await {
        Ok(Some(_)) => {
            return error_response!(
                HttpResponse::Conflict(),
                "EMAIL_EXISTS",
                "Email already registered",
                format!("User with email {} already exists", email)
            )
        }
        Ok(None) => {}
        Err(e) => {
            return error_response!(
                HttpResponse::InternalServerError(),
                "DB_ERROR",
                "Failed to check email",
                e.to_string()
            )
        }
    }

    let user_id = state.snowflake.generate().await.to_string();

    match state.mysql.create_user(&user_id, email, &hashed_password).await {
        Ok(_) => HttpResponse::Created().json(json!({
            "data": {
                "id": user_id,
                "email": email
            }
        })),
        Err(e) => error_response!(
            HttpResponse::InternalServerError(),
            "CREATE_FAILED",
            "Failed to create user",
            e.to_string()
        ),
    }
}

pub async fn get_user_handler(
    state: web::Data<AppState>,
    req: HttpRequest,
) -> HttpResponse {
    let user_id = match basic_auth(state.clone(), req).await {
        Ok(id) => id,
        Err(e) => {
            return error_response!(
                HttpResponse::Unauthorized(),
                "AUTH_FAILED",
                "Authentication failed",
                e.to_string()
            )
        }
    };

    match state.mysql.get_user(&user_id).await {
        Ok(Some(user)) => HttpResponse::Ok().json(json!({
            "data": {
                "id": user.id,
                "email": user.email,
                "icon_url": user.icon_url,
                "created_at": user.created_at
            }
        })),
        Ok(None) => {
            error_response!(
                HttpResponse::NotFound(),
                "USER_NOT_FOUND",
                "User not found",
                ""
            )
        }
        Err(e) => {
            error_response!(
                HttpResponse::InternalServerError(),
                "DB_ERROR",
                "Failed to retrieve user",
                e.to_string()
            )
        }
    }
}