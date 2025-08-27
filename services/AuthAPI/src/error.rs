use axum::{http::StatusCode, response::Json, response::IntoResponse};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("BCrypt error: {0}")]
    BCrypt(#[from] bcrypt::BcryptError),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Bad request: {0}")]
    BadRequest(String),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AuthError::Database(e) => {
                tracing::error!("Database error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error occurred")
            }
            AuthError::Jwt(e) => {
                tracing::error!("JWT error: {}", e);
                (StatusCode::UNAUTHORIZED, "Invalid or expired token")
            }
            AuthError::BCrypt(e) => {
                tracing::error!("BCrypt error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Password processing error")
            }
            AuthError::Validation(ref e) => {
                (StatusCode::BAD_REQUEST, e.as_str())
            }
            AuthError::Unauthorized(ref e) => {
                (StatusCode::UNAUTHORIZED, e.as_str())
            }
            AuthError::NotFound(ref e) => {
                (StatusCode::NOT_FOUND, e.as_str())
            }
            AuthError::Conflict(ref e) => {
                (StatusCode::CONFLICT, e.as_str())
            }
            AuthError::Internal(ref e) => {
                tracing::error!("Internal error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, e.as_str())
            }
            AuthError::BadRequest(ref e) => {
                (StatusCode::BAD_REQUEST, e.as_str())
            }
        };

        let body = Json(json!({
            "error": error_message,
            "status": status.as_u16()
        }));

        (status, body).into_response()
    }
}