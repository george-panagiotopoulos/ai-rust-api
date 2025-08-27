use axum::{
    extract::State,
    http::HeaderMap,
    response::Json,
    routing::{get, post},
    Router,
};
use aws_config::BehaviorVersion;
use aws_sdk_bedrockruntime::{
    Client as BedrockClient, 
    types::{ContentBlock, ConversationRole, ConverseOutput, InferenceConfiguration}
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::env;
use tower_http::cors::CorsLayer;
use tracing::{info, error};
use tracing_subscriber::fmt::init;
use uuid::Uuid;

mod auth_client;
mod error;

use auth_client::AuthClient;
use error::AppError;

#[derive(Clone)]
struct AppState {
    bedrock_client: BedrockClient,
    model_id: String,
    max_tokens: i32,
    auth_client: AuthClient,
}

#[derive(Deserialize)]
struct ChatRequest {
    message: String,
}

#[derive(Serialize)]
struct ChatResponse {
    id: Uuid,
    response: String,
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    service: String,
}

#[derive(Deserialize)]
struct SimpleChatRequest {
    prompt: String,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
    top_p: Option<f32>,
}

#[derive(Serialize)]
struct SimpleChatResponse {
    response: String,
    token_count: Option<u32>,
}

async fn health() -> Result<Json<HealthResponse>, AppError> {
    Ok(Json(HealthResponse {
        status: "healthy".to_string(),
        service: "bedrock-chat-api".to_string(),
    }))
}

async fn chat(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<ChatRequest>,
) -> Result<Json<ChatResponse>, AppError> {
    // Extract token from Authorization header
    let auth_header = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| AppError::AuthError("Missing authorization header".to_string()))?;

    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::AuthError("Invalid authorization header format".to_string()));
    }

    let token = &auth_header[7..];
    
    // Validate token using AuthClient
    let validation_response = state.auth_client.validate_token(token).await?;
    
    if !validation_response.valid {
        return Err(AppError::AuthError("Invalid token".to_string()));
    }
    
    let user_info = validation_response.user
        .ok_or_else(|| AppError::AuthError("No user information in token validation response".to_string()))?;
    
    info!("Received chat request from user {} ({}): {}", user_info.username, user_info.email, payload.message);
    
    let content_block = ContentBlock::Text(payload.message);
    let message = aws_sdk_bedrockruntime::types::Message::builder()
        .role(ConversationRole::User)
        .content(content_block)
        .build()
        .map_err(|e| {
            error!("Failed to build message: {}", e);
            AppError::Internal("Failed to build message".to_string())
        })?;

    let inference_config = aws_sdk_bedrockruntime::types::InferenceConfiguration::builder()
        .max_tokens(state.max_tokens)
        .build();

    let request = state.bedrock_client
        .converse()
        .model_id(&state.model_id)
        .messages(message)
        .inference_config(inference_config);

    match request.send().await {
        Ok(response) => {
            if let Some(output) = response.output {
                match output {
                    aws_sdk_bedrockruntime::types::ConverseOutput::Message(msg) => {
                        if let Some(content) = msg.content.first() {
                            match content {
                                ContentBlock::Text(text) => {
                                    let chat_response = ChatResponse {
                                        id: Uuid::new_v4(),
                                        response: text.clone(),
                                    };
                                    Ok(Json(chat_response))
                                }
                                _ => {
                                    error!("Unexpected content type in response");
                                    Err(AppError::BedrockError("Unexpected content type".to_string()))
                                }
                            }
                        } else {
                            error!("No content in response");
                            Err(AppError::BedrockError("No content in response".to_string()))
                        }
                    }
                    _ => {
                        error!("Unexpected output type");
                        Err(AppError::BedrockError("Unexpected output type".to_string()))
                    }
                }
            } else {
                error!("No output in response");
                Err(AppError::BedrockError("No output in response".to_string()))
            }
        }
        Err(e) => {
            error!("Bedrock API error: {:?}", e);
            Err(AppError::BedrockError(e.to_string()))
        }
    }
}

async fn simple_chat(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<SimpleChatRequest>,
) -> Result<Json<SimpleChatResponse>, AppError> {
    // Extract token from Authorization header
    let auth_header = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| AppError::AuthError("Missing authorization header".to_string()))?;

    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::AuthError("Invalid authorization header format".to_string()));
    }

    let token = &auth_header[7..];
    
    // Validate token using AuthClient
    let validation_response = state.auth_client.validate_token(token).await?;
    if !validation_response.valid {
        return Err(AppError::Unauthorized("Invalid or expired token".to_string()));
    }

    // Use the prompt directly instead of extracting from message
    let content_block = ContentBlock::Text(payload.prompt);
    let message = aws_sdk_bedrockruntime::types::Message::builder()
        .role(ConversationRole::User)
        .content(content_block)
        .build()
        .map_err(|e| {
            error!("Failed to build message: {}", e);
            AppError::Internal("Failed to build message".to_string())
        })?;

    let max_tokens = payload.max_tokens.unwrap_or(state.max_tokens as u32) as i32;

    let mut inference_config_builder = InferenceConfiguration::builder()
        .max_tokens(max_tokens);

    if let Some(temp) = payload.temperature {
        inference_config_builder = inference_config_builder.temperature(temp);
    }

    if let Some(top_p) = payload.top_p {
        inference_config_builder = inference_config_builder.top_p(top_p);
    }

    let inference_config = inference_config_builder.build();

    match state.bedrock_client.converse()
        .model_id(&state.model_id)
        .messages(message)
        .inference_config(inference_config)
        .send()
        .await {
        Ok(response) => {
            if let Some(output) = response.output {
                match output {
                    ConverseOutput::Message(msg) => {
                        if let Some(content) = msg.content.first() {
                            match content {
                                ContentBlock::Text(text) => {
                                    info!("Generated response for simple chat");
                                    Ok(Json(SimpleChatResponse {
                                        response: text.clone(),
                                        token_count: response.usage.map(|u| u.output_tokens as u32),
                                    }))
                                }
                                _ => {
                                    error!("Unexpected content type");
                                    Err(AppError::BedrockError("Unexpected content type".to_string()))
                                }
                            }
                        } else {
                            error!("No content in message");
                            Err(AppError::BedrockError("No content in response".to_string()))
                        }
                    }
                    _ => {
                        error!("Unexpected output type");
                        Err(AppError::BedrockError("Unexpected output type".to_string()))
                    }
                }
            } else {
                error!("No output in response");
                Err(AppError::BedrockError("No output in response".to_string()))
            }
        }
        Err(e) => {
            error!("Bedrock API error: {:?}", e);
            Err(AppError::BedrockError(e.to_string()))
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    init();

    info!("Starting Bedrock Chat API server...");

    let model_id = env::var("BEDROCK_MODEL_ID")
        .unwrap_or_else(|_| "us.anthropic.claude-sonnet-4-20250514-v1:0".to_string());
    
    let max_tokens = env::var("MAX_TOKENS")
        .unwrap_or_else(|_| "4096".to_string())
        .parse::<i32>()
        .unwrap_or(4096);

    let server_host = env::var("SERVER_HOST")
        .unwrap_or_else(|_| "0.0.0.0".to_string());
    
    let server_port = env::var("SERVER_PORT")
        .unwrap_or_else(|_| "3000".to_string());

    let auth_api_url = env::var("AUTH_API_URL")
        .unwrap_or_else(|_| "http://localhost:9102".to_string());
    
    let auth_client = AuthClient::new(auth_api_url);

    let config = aws_config::defaults(BehaviorVersion::latest())
        .load()
        .await;
    
    let bedrock_client = BedrockClient::new(&config);

    let app_state = Arc::new(AppState {
        bedrock_client,
        model_id,
        max_tokens,
        auth_client,
    });

    let app = Router::new()
        .route("/health", get(health))
        .route("/chat", post(chat))
        .route("/simple-chat", post(simple_chat))
        .layer(CorsLayer::permissive())
        .with_state(app_state);

    let bind_address = format!("{}:{}", server_host, server_port);
    let listener = tokio::net::TcpListener::bind(&bind_address).await?;
    info!("Server running on http://{}", bind_address);

    axum::serve(listener, app).await?;

    Ok(())
}
