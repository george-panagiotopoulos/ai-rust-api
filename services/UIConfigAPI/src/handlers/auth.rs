use axum::{extract::State, http::StatusCode, Json, Extension};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{error, info};
use crate::{AppState, middleware::AuthUser};

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub success: bool,
    pub message: String,
    pub user_id: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    pub message: String,
    pub conversation_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ChatResponse {
    pub response: String,
    pub conversation_id: String,
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, StatusCode> {
    info!("Attempting to register user: {}", payload.username);

    match state.db.create_user(&payload.username, &payload.email, &payload.password, false).await {
        Ok(user_id) => {
            info!("Successfully registered user {} with ID {}", payload.username, user_id);
            Ok(Json(RegisterResponse {
                success: true,
                message: "User registered successfully".to_string(),
                user_id: Some(user_id),
            }))
        }
        Err(e) => {
            error!("Failed to register user {}: {}", payload.username, e);
            if e.to_string().contains("unique constraint") {
                Ok(Json(RegisterResponse {
                    success: false,
                    message: "Username or email already exists".to_string(),
                    user_id: None,
                }))
            } else {
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}

pub async fn chat(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Json(payload): Json<ChatRequest>,
) -> Result<Json<ChatResponse>, StatusCode> {
    info!("Processing chat request from user: {}", user.username);

    let conversation_id = payload.conversation_id.unwrap_or_else(|| {
        format!("conv_{}_{}", user.id, chrono::Utc::now().timestamp())
    });

    let bedrock_url = format!("{}/chat", state.config.bedrock_api_url);
    let client = reqwest::Client::new();
    
    let bedrock_request = json!({
        "message": payload.message,
        "conversation_id": conversation_id
    });

    match client.post(&bedrock_url).json(&bedrock_request).send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<serde_json::Value>().await {
                    Ok(bedrock_response) => {
                        let chat_response = bedrock_response.get("response")
                            .and_then(|r| r.as_str())
                            .unwrap_or("Sorry, I couldn't process your request.")
                            .to_string();

                        if let Err(e) = state.db.save_chat_history(
                            user.id,
                            &conversation_id,
                            &payload.message,
                            &chat_response,
                        ).await {
                            error!("Failed to save chat history: {}", e);
                        }

                        Ok(Json(ChatResponse {
                            response: chat_response,
                            conversation_id,
                        }))
                    }
                    Err(e) => {
                        error!("Failed to parse Bedrock response: {}", e);
                        Err(StatusCode::INTERNAL_SERVER_ERROR)
                    }
                }
            } else {
                error!("Bedrock API returned error status: {}", response.status());
                Err(StatusCode::BAD_GATEWAY)
            }
        }
        Err(e) => {
            error!("Failed to connect to Bedrock API: {}", e);
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}