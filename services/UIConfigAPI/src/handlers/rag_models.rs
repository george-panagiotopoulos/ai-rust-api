use axum::{
    extract::{State, Path},
    http::StatusCode,
    Json,
    Extension,
};
use serde_json::json;
use tracing::{info, error, warn};
use crate::{AppState, middleware::{AuthUser, User}, models::{RagModelWithVector, RagModelListResponse, CreateRagModelRequest, UpdateRagModelRequest, RagModelResponse}};

pub async fn list_rag_models(
    State(state): State<AppState>,
    Extension(admin_user): Extension<AuthUser>,
) -> Result<Json<RagModelListResponse>, StatusCode> {
    info!("Admin {} requesting RAG models list", admin_user.username);

    match state.db.list_rag_models().await {
        Ok(rag_models) => {
            let total = rag_models.len();
            Ok(Json(RagModelListResponse { rag_models, total }))
        }
        Err(e) => {
            error!("Failed to fetch RAG models: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn list_rag_models_public(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
) -> Result<Json<RagModelListResponse>, StatusCode> {
    info!("User {} requesting public RAG models list", user.username);

    match state.db.list_rag_models().await {
        Ok(rag_models) => {
            let total = rag_models.len();
            Ok(Json(RagModelListResponse { rag_models, total }))
        }
        Err(e) => {
            error!("Failed to fetch RAG models for user {}: {}", user.username, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn create_rag_model(
    State(state): State<AppState>,
    Extension(admin_user): Extension<AuthUser>,
    Json(payload): Json<CreateRagModelRequest>,
) -> Result<Json<RagModelResponse>, StatusCode> {
    info!("Admin {} creating RAG model: {} with vector ID: {}", admin_user.username, payload.name, payload.vector_id);

    // Validate that the vector exists
    match state.db.get_vector_by_id(payload.vector_id).await {
        Ok(Some(_)) => {
            // Vector exists, proceed with creation
        }
        Ok(None) => {
            return Ok(Json(RagModelResponse {
                success: false,
                message: "Specified vector does not exist".to_string(),
                rag_model: None,
            }));
        }
        Err(e) => {
            error!("Failed to check vector existence: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    // Check if a RAG model with this name already exists
    match state.db.list_rag_models().await {
        Ok(existing_models) => {
            if existing_models.iter().any(|m| m.name == payload.name) {
                return Ok(Json(RagModelResponse {
                    success: false,
                    message: "A RAG model with this name already exists".to_string(),
                    rag_model: None,
                }));
            }
        }
        Err(e) => {
            error!("Failed to check existing RAG models: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    // Create the RAG model
    match state.db.create_rag_model(
        &payload.name,
        payload.vector_id,
        &payload.system_prompt,
        payload.context.as_deref(),
        Some(admin_user.id)
    ).await {
        Ok(model_id) => {
            info!("Created RAG model {} with ID: {}", payload.name, model_id);
            
            // Fetch the created model with vector info
            match state.db.get_rag_model_by_id(model_id).await {
                Ok(Some(rag_model)) => {
                    Ok(Json(RagModelResponse {
                        success: true,
                        message: format!("RAG model '{}' created successfully", payload.name),
                        rag_model: Some(rag_model),
                    }))
                }
                Ok(None) => {
                    warn!("Created RAG model {} but couldn't fetch it", model_id);
                    Ok(Json(RagModelResponse {
                        success: true,
                        message: format!("RAG model '{}' created successfully", payload.name),
                        rag_model: None,
                    }))
                }
                Err(e) => {
                    error!("Failed to fetch created RAG model: {}", e);
                    Ok(Json(RagModelResponse {
                        success: true,
                        message: format!("RAG model '{}' created successfully", payload.name),
                        rag_model: None,
                    }))
                }
            }
        }
        Err(e) => {
            error!("Failed to create RAG model: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_rag_model(
    State(state): State<AppState>,
    Extension(admin_user): Extension<AuthUser>,
    Path(model_id): Path<i32>,
) -> Result<Json<RagModelWithVector>, StatusCode> {
    info!("Admin {} requesting RAG model with ID: {}", admin_user.username, model_id);

    match state.db.get_rag_model_by_id(model_id).await {
        Ok(Some(rag_model)) => Ok(Json(rag_model)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            error!("Failed to fetch RAG model {}: {}", model_id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn update_rag_model(
    State(state): State<AppState>,
    Extension(admin_user): Extension<AuthUser>,
    Path(model_id): Path<i32>,
    Json(payload): Json<UpdateRagModelRequest>,
) -> Result<Json<RagModelResponse>, StatusCode> {
    info!("Admin {} updating RAG model with ID: {}", admin_user.username, model_id);

    // Check if the RAG model exists
    match state.db.get_rag_model_by_id(model_id).await {
        Ok(Some(_)) => {
            // RAG model exists, proceed with update
        }
        Ok(None) => {
            return Err(StatusCode::NOT_FOUND);
        }
        Err(e) => {
            error!("Failed to check RAG model existence: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    // If vector_id is being updated, validate the new vector exists
    if let Some(vector_id) = payload.vector_id {
        match state.db.get_vector_by_id(vector_id).await {
            Ok(Some(_)) => {
                // Vector exists
            }
            Ok(None) => {
                return Ok(Json(RagModelResponse {
                    success: false,
                    message: "Specified vector does not exist".to_string(),
                    rag_model: None,
                }));
            }
            Err(e) => {
                error!("Failed to check vector existence: {}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
    }

    // Update the RAG model
    match state.db.update_rag_model(
        model_id,
        payload.name.as_deref(),
        payload.vector_id,
        payload.system_prompt.as_deref(),
        payload.context.as_deref(),
        payload.is_active,
    ).await {
        Ok(()) => {
            info!("Successfully updated RAG model ID: {}", model_id);
            
            // Fetch the updated model
            match state.db.get_rag_model_by_id(model_id).await {
                Ok(Some(rag_model)) => {
                    Ok(Json(RagModelResponse {
                        success: true,
                        message: "RAG model updated successfully".to_string(),
                        rag_model: Some(rag_model),
                    }))
                }
                Ok(None) => {
                    warn!("Updated RAG model {} but couldn't fetch it", model_id);
                    Ok(Json(RagModelResponse {
                        success: true,
                        message: "RAG model updated successfully".to_string(),
                        rag_model: None,
                    }))
                }
                Err(e) => {
                    error!("Failed to fetch updated RAG model: {}", e);
                    Ok(Json(RagModelResponse {
                        success: true,
                        message: "RAG model updated successfully".to_string(),
                        rag_model: None,
                    }))
                }
            }
        }
        Err(e) => {
            error!("Failed to update RAG model {}: {}", model_id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete_rag_model(
    State(state): State<AppState>,
    Extension(admin_user): Extension<AuthUser>,
    Path(model_id): Path<i32>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    info!("Admin {} deleting RAG model with ID: {}", admin_user.username, model_id);

    // Check if RAG model exists
    match state.db.get_rag_model_by_id(model_id).await {
        Ok(Some(_)) => {
            // RAG model exists, proceed with deletion
            match state.db.delete_rag_model(model_id).await {
                Ok(()) => {
                    info!("Successfully deleted RAG model ID: {}", model_id);
                    Ok(Json(json!({
                        "success": true,
                        "message": "RAG model deleted successfully"
                    })))
                }
                Err(e) => {
                    error!("Failed to delete RAG model {}: {}", model_id, e);
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
        Ok(None) => {
            warn!("Attempted to delete non-existent RAG model ID: {}", model_id);
            Err(StatusCode::NOT_FOUND)
        }
        Err(e) => {
            error!("Failed to check RAG model existence: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}