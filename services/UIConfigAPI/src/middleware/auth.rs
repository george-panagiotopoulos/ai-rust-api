use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use tracing::{error, warn};

#[derive(Debug, Clone)]
pub struct AuthClient {
    client: Client,
    auth_api_url: String,
}

#[derive(Debug, Serialize)]
struct TokenValidationRequest {
    token: String,
}

#[derive(Debug, Deserialize)]
struct TokenValidationResponse {
    valid: bool,
    user: Option<UserInfo>,
}

#[derive(Debug, Deserialize)]
pub struct UserInfo {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub is_active: bool,
    pub is_admin: bool,
}

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub is_admin: bool,
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub is_admin: bool,
}

impl AuthClient {
    pub fn new(auth_api_url: String) -> Self {
        Self {
            client: Client::new(),
            auth_api_url,
        }
    }

    pub async fn validate_token(&self, token: &str) -> Result<Option<UserInfo>, reqwest::Error> {
        let request = TokenValidationRequest {
            token: token.to_string(),
        };

        let response = self
            .client
            .post(&format!("{}/validate", self.auth_api_url))
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let validation_response: TokenValidationResponse = response.json().await?;
            Ok(validation_response.user)
        } else {
            Ok(None)
        }
    }
}

pub async fn auth_middleware(
    State(auth_client): State<Arc<AuthClient>>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let headers = request.headers();
    
    let auth_header = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if !auth_header.starts_with("Bearer ") {
        warn!("Invalid authorization header format");
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = &auth_header[7..];

    match auth_client.validate_token(token).await {
        Ok(Some(user_info)) => {
            let auth_user = AuthUser {
                id: user_info.id,
                username: user_info.username,
                email: user_info.email,
                is_admin: user_info.is_admin,
            };
            request.extensions_mut().insert(auth_user);
            Ok(next.run(request).await)
        }
        Ok(None) => {
            warn!("Invalid token provided");
            Err(StatusCode::UNAUTHORIZED)
        }
        Err(e) => {
            error!("Failed to validate token: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn admin_middleware(
    mut request: Request,
    next: Next,
) -> Result<Response, Response> {
    let auth_user = request
        .extensions()
        .get::<AuthUser>()
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": "Authentication required" }))
            ).into_response()
        })?;

    if !auth_user.is_admin {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "Admin access required" }))
        ).into_response());
    }

    Ok(next.run(request).await)
}

pub async fn require_admin(
    State(auth_client): State<Arc<AuthClient>>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let headers = request.headers();
    
    let auth_header = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if !auth_header.starts_with("Bearer ") {
        warn!("Invalid authorization header format");
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = &auth_header[7..];

    match auth_client.validate_token(token).await {
        Ok(Some(user_info)) => {
            if !user_info.is_admin {
                warn!("Non-admin user {} attempted to access admin endpoint", user_info.username);
                return Err(StatusCode::FORBIDDEN);
            }
            
            let auth_user = AuthUser {
                id: user_info.id,
                username: user_info.username,
                email: user_info.email,
                is_admin: user_info.is_admin,
            };
            request.extensions_mut().insert(auth_user);
            Ok(next.run(request).await)
        }
        Ok(None) => {
            warn!("Invalid token provided");
            Err(StatusCode::UNAUTHORIZED)
        }
        Err(e) => {
            error!("Failed to validate token: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn require_user(
    State(auth_client): State<Arc<AuthClient>>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let headers = request.headers();
    
    let auth_header = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if !auth_header.starts_with("Bearer ") {
        warn!("Invalid authorization header format");
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = &auth_header[7..];

    match auth_client.validate_token(token).await {
        Ok(Some(user_info)) => {
            let user = User {
                id: user_info.id,
                username: user_info.username,
                email: user_info.email,
                is_admin: user_info.is_admin,
            };
            request.extensions_mut().insert(user);
            Ok(next.run(request).await)
        }
        Ok(None) => {
            warn!("Invalid token provided");
            Err(StatusCode::UNAUTHORIZED)
        }
        Err(e) => {
            error!("Failed to validate token: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}