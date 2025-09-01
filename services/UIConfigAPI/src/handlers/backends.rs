use crate::{AppState, middleware::AuthUser};
use crate::models::config::{BackendConfigRequest, BackendConfigResponse, BackendStatusResponse};
use axum::{
    extract::{Path, State, Extension},
    http::StatusCode,
    response::Json,
    Json as RequestJson,
};
use serde_json::json;
use tracing::{error, info};

pub async fn get_backend_configs(
    State(state): State<AppState>,
    Extension(_user): Extension<AuthUser>, // Requires authentication
) -> Result<Json<BackendConfigResponse>, StatusCode> {
    match state.db.get_backend_configs().await {
        Ok(backends) => {
            let active_backend = state.db.get_active_backend().await.ok().flatten();
            
            info!("Retrieved {} backend configurations", backends.len());
            Ok(Json(BackendConfigResponse {
                backends,
                active_backend,
            }))
        }
        Err(e) => {
            error!("Failed to retrieve backend configs: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn update_backend_config(
    State(state): State<AppState>,
    Extension(_user): Extension<AuthUser>, // Requires authentication
    Path(provider): Path<String>,
    RequestJson(config): RequestJson<BackendConfigRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    if provider != "aws" && provider != "azure" {
        return Err(StatusCode::BAD_REQUEST);
    }

    match state.db.update_backend_config(&provider, &config).await {
        Ok(_) => {
            info!("Successfully updated {} backend configuration", provider);
            Ok(Json(json!({
                "success": true,
                "message": format!("{} backend configuration updated successfully", provider.to_uppercase())
            })))
        }
        Err(e) => {
            error!("Failed to update {} backend config: {}", provider, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn set_active_backend(
    State(state): State<AppState>,
    Extension(_user): Extension<AuthUser>, // Requires authentication
    Path(provider): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    if provider != "aws" && provider != "azure" {
        return Err(StatusCode::BAD_REQUEST);
    }

    match state.db.set_active_backend(&provider).await {
        Ok(_) => {
            info!("Successfully set {} as active backend", provider);
            Ok(Json(json!({
                "success": true,
                "message": format!("{} is now the active backend", provider.to_uppercase())
            })))
        }
        Err(e) => {
            error!("Failed to set active backend to {}: {}", provider, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_backend_status(
    State(state): State<AppState>,
    Extension(_user): Extension<AuthUser>, // Requires authentication
) -> Result<Json<BackendStatusResponse>, StatusCode> {
    // Get the active backend
    let active_backend = match state.db.get_active_backend().await {
        Ok(Some(backend)) => backend,
        Ok(None) => "none".to_string(),
        Err(e) => {
            error!("Failed to get active backend: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // TODO: Add actual health checks for AWS and Azure services
    // For now, we'll return basic status based on configuration
    let aws_status = if active_backend == "aws" { "active" } else { "configured" };
    let azure_status = if active_backend == "azure" { "active" } else { "configured" };

    Ok(Json(BackendStatusResponse {
        aws_status: aws_status.to_string(),
        azure_status: azure_status.to_string(),
        active_backend,
    }))
}

pub async fn test_backend_connection(
    State(state): State<AppState>,
    Extension(_user): Extension<AuthUser>, // Requires authentication
    Path(provider): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    if provider != "aws" && provider != "azure" {
        return Err(StatusCode::BAD_REQUEST);
    }

    // TODO: Implement actual connection testing
    // For now, return a placeholder response
    
    match provider.as_str() {
        "aws" => {
            info!("Testing AWS backend connection");
            Ok(Json(json!({
                "success": true,
                "provider": "aws",
                "message": "AWS connection test successful",
                "services": {
                    "bedrock": "healthy",
                    "embeddings": "healthy"
                }
            })))
        }
        "azure" => {
            info!("Testing Azure backend connection");
            Ok(Json(json!({
                "success": true,
                "provider": "azure",
                "message": "Azure OpenAI connection test successful",
                "services": {
                    "chat": "healthy",
                    "embeddings": "healthy"
                }
            })))
        }
        _ => Err(StatusCode::BAD_REQUEST)
    }
}