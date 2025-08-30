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
        let (status, error_code, error_message) = match self {
            AuthError::Database(e) => {
                tracing::error!("Database error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "DATABASE_ERROR", "Database error occurred")
            }
            AuthError::Jwt(e) => {
                tracing::error!("JWT error: {}", e);
                (StatusCode::UNAUTHORIZED, "JWT_ERROR", "Invalid or expired token")
            }
            AuthError::BCrypt(e) => {
                tracing::error!("BCrypt error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "BCRYPT_ERROR", "Password processing error")
            }
            AuthError::Validation(ref e) => {
                (StatusCode::BAD_REQUEST, "VALIDATION_ERROR", e.as_str())
            }
            AuthError::Unauthorized(ref e) => {
                (StatusCode::UNAUTHORIZED, "UNAUTHORIZED", e.as_str())
            }
            AuthError::NotFound(ref e) => {
                (StatusCode::NOT_FOUND, "NOT_FOUND", e.as_str())
            }
            AuthError::Conflict(ref e) => {
                (StatusCode::CONFLICT, "CONFLICT", e.as_str())
            }
            AuthError::Internal(ref e) => {
                tracing::error!("Internal error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", e.as_str())
            }
            AuthError::BadRequest(ref e) => {
                (StatusCode::BAD_REQUEST, "BAD_REQUEST", e.as_str())
            }
        };

        let body = Json(json!({
            "error": {
                "code": error_code,
                "message": error_message,
                "timestamp": chrono::Utc::now().to_rfc3339()
            }
        }));

        (status, body).into_response()
    }
}