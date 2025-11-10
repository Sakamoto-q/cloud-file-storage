use crate::router::AppState;
use std::error::Error;
use crate::database::Session;
use actix_web::{web, HttpRequest};

pub fn get_client_ip(req: &HttpRequest) -> String {
    if let Some(forwarded) = req.headers().get("X-Forwarded-For") {
        if let Ok(forwarded_str) = forwarded.to_str() {
            if let Some(ip) = forwarded_str.split(',').next() {
                return ip.trim().to_string();
            }
        }
    }
    if let Some(real_ip) = req.headers().get("X-Real-IP") {
        if let Ok(ip_str) = real_ip.to_str() {
            return ip_str.to_string();
        }
    }
    req.peer_addr()
        .map(|addr| addr.ip().to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

pub async fn bearer_auth(
    state: web::Data<AppState>,
    req: &HttpRequest,
) -> Result<Session, Box<dyn Error>> {
    let session_key = if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                auth_str[7..].to_string()
            } else {
                return Err("Missing Bearer token".into());
            }
        } else {
            return Err("Invalid Authorization header format".into());
        }
    } else {
        return Err("Missing Authorization header".into());
    };

    let ip_address = get_client_ip(req);
    
    match state.mysql.verify_session(&session_key, &ip_address).await {
        Ok(Some(session)) => Ok(session),
        Ok(None) => Err("Invalid session or IP mismatch".into()),
        Err(e) => Err(e.to_string().into()),
    }
}