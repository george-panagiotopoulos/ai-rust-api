use crate::auth_client::AuthClient;
use crate::azure_client::{AzureClient, ChatRequest, SimpleChatRequest};
use crate::error::AppError;
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::Json,
    Json as RequestJson,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{error, info};

#[derive(Clone)]
pub struct AppState {
    pub azure_client: Arc<AzureClient>,
    pub auth_client: Arc<AuthClient>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
    pub version: String,
}

async fn extract_and_validate_token(auth_client: &AuthClient, headers: &HeaderMap) -> Result<String, AppError> {
    // Extract token from Authorization header
    let auth_header = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| AppError::Unauthorized("Missing authorization header".to_string()))?;

    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::Unauthorized("Invalid authorization header format".to_string()));
    }

    let token = &auth_header[7..];

    // Validate token
    match auth_client.validate_token(token).await {
        Ok(validation_response) if validation_response.valid => {
            Ok(token.to_string())
        }
        Ok(_) => Err(AppError::Unauthorized("Invalid or expired token".to_string())),
        Err(e) => {
            error!("Token validation error: {}", e);
            Err(AppError::Internal("Failed to validate token".to_string()))
        }
    }
}

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        service: "AzureAPI".to_string(),
        version: "0.1.0".to_string(),
    })
}

pub async fn chat(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    RequestJson(request): RequestJson<ChatRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    // Extract and validate authentication token
    let _token = extract_and_validate_token(&app_state.auth_client, &headers).await?;

    info!("Received chat request: {}", request.message);

    match app_state.azure_client.chat_completion(request).await {
        Ok(response) => {
            info!("Successfully processed chat request");
            Ok(Json(serde_json::json!({
                "id": response.id,
                "response": response.response
            })))
        }
        Err(e) => {
            error!("Failed to process chat request: {}", e);
            Err(e)
        }
    }
}

pub async fn simple_chat(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    RequestJson(request): RequestJson<SimpleChatRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    // Extract and validate authentication token
    let _token = extract_and_validate_token(&app_state.auth_client, &headers).await?;

    info!("Received simple chat request: {}", request.prompt);

    match app_state.azure_client.simple_chat(request).await {
        Ok(response) => {
            info!("Successfully processed simple chat request");
            Ok(Json(serde_json::json!({
                "response": response.response,
                "token_count": response.token_count
            })))
        }
        Err(e) => {
            error!("Failed to process simple chat request: {}", e);
            Err(e)
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbeddingRequest {
    pub text: String,
}

pub async fn create_embedding(
    State(app_state): State<AppState>,
    RequestJson(request): RequestJson<EmbeddingRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    info!("Received embedding request for text (length: {})", request.text.len());

    match app_state.azure_client.create_embedding(&request.text).await {
        Ok(embedding) => {
            let dimension = embedding.len();
            info!("Successfully generated embedding with dimension: {}", dimension);
            Ok(Json(serde_json::json!({
                "embedding": embedding,
                "dimension": dimension
            })))
        }
        Err(e) => {
            error!("Failed to generate embedding: {}", e);
            Err(e)
        }
    }
}