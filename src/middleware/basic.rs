use crate::router::AppState;
use std::error::Error;
use base64::{engine::general_purpose, Engine as _};
use actix_web::{web, HttpRequest};
use crate::encrypt::Bcrypt;

pub async fn basic_auth(
    state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<String, Box<dyn Error>> {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Basic ") {
                let credentials = &auth_str[6..];
                
                match general_purpose::STANDARD.decode(credentials) {
                    Ok(decoded) => {
                        if let Ok(auth_string) = String::from_utf8(decoded) {
                            let parts: Vec<&str> = auth_string.split(':').collect();
                            
                            if parts.len() == 2 {
                                let useremail = parts[0];
                                let password = parts[1];
                                
                                match state.mysql.get_user_by_email(useremail).await {
                                    Ok(Some(user)) => {
                                        if Bcrypt::verify(password, &user.password_hash)? {
                                            return Ok(user.id);
                                        } else {
                                            return Err("Invalid password".into());
                                        }
                                    }
                                    Ok(None) => {
                                        return Err("User not found".into());
                                    }
                                    Err(e) => {
                                        return Err(e.to_string().into());
                                    }
                                }
                            } else {
                                return Err("Invalid credentials format".into());
                            }
                        } else {
                            return Err("Failed to decode credentials as UTF-8".into());
                        }
                    }
                    Err(_) => {
                        return Err("Failed to decode Base64 credentials".into());
                    }
                }
            } else {
                return Err("Authorization header must start with 'Basic '".into());
            }
        } else {
            return Err("Invalid Authorization header format".into());
        }
    } else {
        return Err("Missing Authorization header".into());
    }
}