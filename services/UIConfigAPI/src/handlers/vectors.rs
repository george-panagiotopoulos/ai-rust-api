use axum::{
    extract::{State, Path},
    http::StatusCode,
    Json,
    Extension,
};
use serde_json::json;
use tracing::{info, error, warn};
use crate::{AppState, middleware::AuthUser, models::{Vector, VectorListResponse, CreateVectorRequest, VectorCreationResponse}};

pub async fn list_vectors(
    State(state): State<AppState>,
    Extension(admin_user): Extension<AuthUser>,
) -> Result<Json<VectorListResponse>, StatusCode> {
    info!("Admin {} requesting vectors list", admin_user.username);

    match state.db.list_vectors().await {
        Ok(vectors) => {
            let total = vectors.len();
            Ok(Json(VectorListResponse { vectors, total }))
        }
        Err(e) => {
            error!("Failed to fetch vectors: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn create_vector(
    State(state): State<AppState>,
    Extension(admin_user): Extension<AuthUser>,
    Json(payload): Json<CreateVectorRequest>,
) -> Result<Json<VectorCreationResponse>, StatusCode> {
    info!("Admin {} creating vector: {} for folder: {}", admin_user.username, payload.name, payload.folder_name);

    // Validate that the folder exists
    let documents_path = std::path::Path::new(&state.config.documents_base_path)
        .join(&payload.folder_name);
    
    if !documents_path.exists() || !documents_path.is_dir() {
        return Ok(Json(VectorCreationResponse {
            success: false,
            message: "Document folder does not exist".to_string(),
            vector_id: None,
            processing_status: "failed".to_string(),
        }));
    }

    // Check if a vector with this name already exists
    match state.db.list_vectors().await {
        Ok(existing_vectors) => {
            if existing_vectors.iter().any(|v| v.name == payload.name) {
                return Ok(Json(VectorCreationResponse {
                    success: false,
                    message: "A vector with this name already exists".to_string(),
                    vector_id: None,
                    processing_status: "failed".to_string(),
                }));
            }
        }
        Err(e) => {
            error!("Failed to check existing vectors: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    // Create the vector record in database
    match state.db.create_vector(
        &payload.name,
        &payload.folder_name,
        payload.description.as_deref(),
        Some(admin_user.id)
    ).await {
        Ok(vector_id) => {
            info!("Created vector {} with ID: {}", payload.name, vector_id);
            
            // Trigger background vector processing
            let processor = crate::handlers::vector_processing::VectorProcessor::new(
                state.config.documents_base_path.clone(),
                "http://127.0.0.1:9101".to_string(), // RAGAPI URL
            );
            
            let db_clone = state.db.clone();
            let folder_name_clone = payload.folder_name.clone();
            
            // Spawn background task for vector processing
            tokio::spawn(async move {
                crate::handlers::vector_processing::run_vector_processing_task(
                    processor,
                    vector_id,
                    folder_name_clone,
                    db_clone,
                ).await;
            });
            
            info!("Background vector processing task spawned for vector ID: {}", vector_id);
            Ok(Json(VectorCreationResponse {
                success: true,
                message: format!("Vector '{}' created successfully. Processing will begin shortly.", payload.name),
                vector_id: Some(vector_id),
                processing_status: "queued".to_string(),
            }))
        }
        Err(e) => {
            error!("Failed to create vector: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete_vector(
    State(state): State<AppState>,
    Extension(admin_user): Extension<AuthUser>,
    Path(vector_id): Path<i32>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    info!("Admin {} deleting vector with ID: {}", admin_user.username, vector_id);

    // Check if vector exists
    match state.db.get_vector_by_id(vector_id).await {
        Ok(Some(_)) => {
            // Vector exists, proceed with deletion
            match state.db.delete_vector(vector_id).await {
                Ok(()) => {
                    info!("Successfully deleted vector ID: {}", vector_id);
                    Ok(Json(json!({
                        "success": true,
                        "message": "Vector deleted successfully"
                    })))
                }
                Err(e) => {
                    error!("Failed to delete vector {}: {}", vector_id, e);
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
        Ok(None) => {
            warn!("Attempted to delete non-existent vector ID: {}", vector_id);
            Err(StatusCode::NOT_FOUND)
        }
        Err(e) => {
            error!("Failed to check vector existence: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_vector(
    State(state): State<AppState>,
    Extension(admin_user): Extension<AuthUser>,
    Path(vector_id): Path<i32>,
) -> Result<Json<Vector>, StatusCode> {
    info!("Admin {} requesting vector with ID: {}", admin_user.username, vector_id);

    match state.db.get_vector_by_id(vector_id).await {
        Ok(Some(vector)) => Ok(Json(vector)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            error!("Failed to fetch vector {}: {}", vector_id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}