use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;
use tracing::error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Authentication failed: {0}")]
    AuthError(String),
    
    #[error("Invalid request: {0}")]
    BadRequest(String),
    
    #[error("Resource not found: {0}")]
    NotFound(String),
    
    #[error("Internal server error: {0}")]
    Internal(String),
    
    #[error("AWS Bedrock error: {0}")]
    BedrockError(String),
    
    #[error("Token generation error: {0}")]
    TokenError(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message, error_code) = match &self {
            AppError::AuthError(msg) => {
                (StatusCode::UNAUTHORIZED, msg.clone(), "AUTH_ERROR")
            }
            AppError::BadRequest(msg) => {
                (StatusCode::BAD_REQUEST, msg.clone(), "BAD_REQUEST")
            }
            AppError::NotFound(msg) => {
                (StatusCode::NOT_FOUND, msg.clone(), "NOT_FOUND")
            }
            AppError::Internal(msg) => {
                error!("Internal server error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string(), "INTERNAL_ERROR")
            }
            AppError::BedrockError(msg) => {
                error!("Bedrock error: {}", msg);
                (StatusCode::BAD_GATEWAY, "External service error".to_string(), "BEDROCK_ERROR")
            }
            AppError::TokenError(msg) => {
                error!("Token error: {}", msg);
                (StatusCode::UNAUTHORIZED, msg.clone(), "TOKEN_ERROR")
            }
            AppError::ValidationError(msg) => {
                (StatusCode::UNPROCESSABLE_ENTITY, msg.clone(), "VALIDATION_ERROR")
            }
            AppError::Unauthorized(msg) => {
                (StatusCode::UNAUTHORIZED, msg.clone(), "UNAUTHORIZED")
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

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        AppError::TokenError(err.to_string())
    }
}

impl From<aws_sdk_bedrockruntime::Error> for AppError {
    fn from(err: aws_sdk_bedrockruntime::Error) -> Self {
        AppError::BedrockError(err.to_string())
    }
}