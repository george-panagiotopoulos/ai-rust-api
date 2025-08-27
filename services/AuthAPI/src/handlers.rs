use crate::{
    auth::JwtManager,
    database::Database,
    error::AuthError,
    models::*,
};
use axum::{
    extract::{ConnectInfo, State},
    http::HeaderMap,
    response::Json,
};
use sha2::{Digest, Sha256};
use std::{net::SocketAddr, sync::Arc};
use tracing::{info, warn};
use validator::Validate;

pub struct AppState {
    pub db: Database,
    pub jwt_manager: JwtManager,
    pub bcrypt_cost: u32,
}

pub async fn health() -> Result<Json<HealthResponse>, AuthError> {
    Ok(Json(HealthResponse {
        status: "healthy".to_string(),
        service: "Authentication API".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    }))
}

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<UserResponse>, AuthError> {
    // Validate input
    payload.validate().map_err(|e| AuthError::Validation(e.to_string()))?;

    info!("Registration attempt for username: {}", payload.username);

    // Hash password
    let password_hash = state.jwt_manager.hash_password(&payload.password, state.bcrypt_cost)?;

    // Create user
    let user = state.db.create_user(&payload.username, &payload.email, &password_hash).await?;

    info!("User registered successfully: {}", user.username);

    Ok(Json(user.into()))
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AuthError> {
    // Validate input
    payload.validate().map_err(|e| AuthError::Validation(e.to_string()))?;

    info!("Login attempt for username: {}", payload.username);

    // Get user
    let user = state
        .db
        .get_user_by_username(&payload.username)
        .await?
        .ok_or_else(|| AuthError::Unauthorized("Invalid username or password".to_string()))?;

    // Verify password
    let password_valid = state.jwt_manager.verify_password(&payload.password, &user.password_hash)?;
    if !password_valid {
        warn!("Invalid password attempt for user: {}", payload.username);
        return Err(AuthError::Unauthorized("Invalid username or password".to_string()));
    }

    // Create JWT token
    let (token, expires_at) = state.jwt_manager.create_token(&user)?;

    // Hash token for storage
    let token_hash = format!("{:x}", Sha256::digest(token.as_bytes()));

    // Extract user agent and IP
    let user_agent = headers
        .get("user-agent")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());
    let ip_address_str = addr.ip().to_string();
    let ip_address = Some(ip_address_str.as_str());

    // Create session
    state.db.create_session(
        user.id,
        &token_hash,
        expires_at,
        user_agent.as_deref(),
        ip_address,
    ).await?;

    // Update last login
    state.db.update_last_login(user.id).await?;

    let expires_in = (expires_at - chrono::Utc::now()).num_seconds();

    info!("User logged in successfully: {}", user.username);

    Ok(Json(LoginResponse {
        access_token: token,
        token_type: "Bearer".to_string(),
        expires_in,
        user: user.into(),
    }))
}

pub async fn logout(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<serde_json::Value>, AuthError> {
    // Extract token from Authorization header
    let auth_header = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| AuthError::BadRequest("Missing authorization header".to_string()))?;

    if !auth_header.starts_with("Bearer ") {
        return Err(AuthError::BadRequest("Invalid authorization header format".to_string()));
    }

    let token = &auth_header[7..];
    let token_hash = format!("{:x}", Sha256::digest(token.as_bytes()));

    // Delete session
    state.db.delete_session(&token_hash).await?;

    info!("User logged out successfully");

    Ok(Json(serde_json::json!({
        "message": "Logged out successfully"
    })))
}

pub async fn validate_token(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<TokenValidationRequest>,
) -> Result<Json<TokenValidationResponse>, AuthError> {
    // Validate JWT token
    let claims = match state.jwt_manager.validate_token(&payload.token) {
        Ok(claims) => claims,
        Err(_) => {
            return Ok(Json(TokenValidationResponse {
                valid: false,
                user: None,
                expires_at: None,
            }));
        }
    };

    // Check if session exists and is not expired
    let token_hash = format!("{:x}", Sha256::digest(payload.token.as_bytes()));
    let session = state.db.get_session_by_token_hash(&token_hash).await?;

    if session.is_none() {
        return Ok(Json(TokenValidationResponse {
            valid: false,
            user: None,
            expires_at: None,
        }));
    }

    // Get user details
    let user_id: i32 = claims.sub.parse().map_err(|_| AuthError::Internal("Invalid user ID in token".to_string()))?;
    let user = state.db.get_user_by_id(user_id).await?;

    match user {
        Some(user) => {
            let session = session.unwrap(); // Safe because we checked above
            Ok(Json(TokenValidationResponse {
                valid: true,
                user: Some(user.into()),
                expires_at: Some(session.expires_at),
            }))
        }
        None => Ok(Json(TokenValidationResponse {
            valid: false,
            user: None,
            expires_at: None,
        }))
    }
}

pub async fn get_profile(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<UserResponse>, AuthError> {
    // Extract and validate token
    let auth_header = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| AuthError::Unauthorized("Missing authorization header".to_string()))?;

    if !auth_header.starts_with("Bearer ") {
        return Err(AuthError::Unauthorized("Invalid authorization header format".to_string()));
    }

    let token = &auth_header[7..];
    let claims = state.jwt_manager.validate_token(token)?;

    // Get user
    let user_id: i32 = claims.sub.parse().map_err(|_| AuthError::Internal("Invalid user ID in token".to_string()))?;
    let user = state
        .db
        .get_user_by_id(user_id)
        .await?
        .ok_or_else(|| AuthError::NotFound("User not found".to_string()))?;

    Ok(Json(user.into()))
}

pub async fn stats(
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, AuthError> {
    let user_count = state.db.get_user_count().await?;
    let session_count = state.db.get_session_count().await?;
    
    // Clean up expired sessions
    let cleaned_sessions = state.db.cleanup_expired_sessions().await?;
    
    if cleaned_sessions > 0 {
        info!("Cleaned up {} expired sessions", cleaned_sessions);
    }

    Ok(Json(serde_json::json!({
        "active_users": user_count,
        "active_sessions": session_count,
        "cleaned_sessions": cleaned_sessions
    })))
}