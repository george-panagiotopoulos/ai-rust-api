use crate::auth_client::AuthClient;
use crate::bedrock_client::RAGRequest;
use crate::rag::{RAGResponse, RAGService, RAGStats};
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::Json,
    Json as RequestJson,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{error, info};

// Helper function to extract and validate authentication token
async fn extract_and_validate_token(auth_client: &AuthClient, headers: &HeaderMap) -> Result<String, (StatusCode, Json<ErrorResponse>)> {
    // Extract token from Authorization header
    let auth_header = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    error: ErrorDetails {
                        code: "MISSING_AUTH_HEADER".to_string(),
                        message: "Missing authorization header".to_string(),
                        timestamp: chrono::Utc::now().to_rfc3339(),
                    },
                }),
            )
        })?;

    if !auth_header.starts_with("Bearer ") {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                error: ErrorDetails {
                    code: "INVALID_AUTH_FORMAT".to_string(),
                    message: "Invalid authorization header format".to_string(),
                    timestamp: chrono::Utc::now().to_rfc3339(),
                },
            }),
        ));
    }

    let token = &auth_header[7..];

    // Validate token
    match auth_client.validate_token(token).await {
        Ok(validation_response) if validation_response.valid => {
            Ok(token.to_string())
        }
        Ok(_) => Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                error: ErrorDetails {
                    code: "INVALID_TOKEN".to_string(),
                    message: "Invalid or expired token".to_string(),
                    timestamp: chrono::Utc::now().to_rfc3339(),
                },
            }),
        )),
        Err(e) => {
            error!("Token validation error: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: ErrorDetails {
                        code: "VALIDATION_ERROR".to_string(),
                        message: "Failed to validate token".to_string(),
                        timestamp: chrono::Utc::now().to_rfc3339(),
                    },
                }),
            ))
        }
    }
}

// Legacy validation function for backwards compatibility
async fn validate_auth_token(auth_client: &AuthClient, headers: &HeaderMap) -> Result<(), (StatusCode, Json<ErrorResponse>)> {
    extract_and_validate_token(auth_client, headers).await.map(|_| ())
}

#[derive(Clone)]
pub struct AppState {
    pub rag_service: Arc<RAGService>,
    pub auth_client: Arc<AuthClient>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: ErrorDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorDetails {
    pub code: String,
    pub message: String,
    pub timestamp: String,
}


pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        service: "RAGAPI".to_string(),
        version: "0.1.0".to_string(),
    })
}

pub async fn stats(
    State(app_state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<RAGStats>, (StatusCode, Json<ErrorResponse>)> {
    // Validate authentication
    validate_auth_token(&app_state.auth_client, &headers).await?;

    match app_state.rag_service.get_stats().await {
        Ok(stats) => {
            info!("Retrieved stats: {:?}", stats);
            Ok(Json(stats))
        }
        Err(e) => {
            error!("Failed to get stats: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: ErrorDetails {
                        code: "STATS_FAILED".to_string(),
                        message: e.to_string(),
                        timestamp: chrono::Utc::now().to_rfc3339(),
                    },
                }),
            ))
        }
    }
}

pub async fn query(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    RequestJson(request): RequestJson<RAGRequest>,
) -> Result<Json<RAGResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Extract and validate authentication token
    let token = extract_and_validate_token(&app_state.auth_client, &headers).await?;

    info!("Received RAG query: {}", request.query);

    match app_state.rag_service.query(request, &token).await {
        Ok(response) => {
            info!("Successfully processed query");
            Ok(Json(response))
        }
        Err(e) => {
            error!("Failed to process query: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: ErrorDetails {
                        code: "QUERY_FAILED".to_string(),
                        message: e.to_string(),
                        timestamp: chrono::Utc::now().to_rfc3339(),
                    },
                }),
            ))
        }
    }
}

pub async fn search_documents(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    RequestJson(request): RequestJson<SearchRequest>,
) -> Result<Json<SearchResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Extract and validate authentication token
    let token = extract_and_validate_token(&app_state.auth_client, &headers).await?;

    info!("Received document search: {}", request.query);
    
    // Create a temporary RAG request to generate embedding
    let temp_request = RAGRequest {
        query: request.query.clone(),
        system_prompt: None,
        context: None,
        max_tokens: None,
        temperature: None,
        rag_model_name: None,
    };

    // Use the RAG service's internal methods
    // For now, we'll return a simplified search response
    match app_state.rag_service.query(temp_request, &token).await {
        Ok(rag_response) => {
            let search_response = SearchResponse {
                documents: rag_response.sources.into_iter().map(|source| {
                    crate::database::DocumentWithSimilarity {
                        document: crate::database::Document {
                            id: 0, // This would need to be properly populated
                            filename: source.filename,
                            content: source.snippet,
                            file_hash: "".to_string(),
                            chunk_index: source.chunk_index,
                            created_at: chrono::Utc::now(),
                            updated_at: chrono::Utc::now(),
                        },
                        similarity: source.similarity,
                    }
                }).collect(),
            };
            Ok(Json(search_response))
        }
        Err(e) => {
            error!("Failed to search documents: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: ErrorDetails {
                        code: "SEARCH_FAILED".to_string(),
                        message: e.to_string(),
                        timestamp: chrono::Utc::now().to_rfc3339(),
                    },
                }),
            ))
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct SearchRequest {
    pub query: String,
    pub limit: Option<i32>,
    pub similarity_threshold: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct SearchResponse {
    pub documents: Vec<crate::database::DocumentWithSimilarity>,
}

#[derive(Debug, Deserialize)]
pub struct ProcessDocumentRequest {
    pub filename: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct ProcessDocumentResponse {
    pub success: bool,
    pub document_id: Option<i32>,
    pub chunks_processed: Option<i32>,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct GenerateEmbeddingRequest {
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct GenerateEmbeddingResponse {
    pub embedding: Vec<f32>,
    pub dimension: usize,
}

pub async fn generate_embedding(
    State(app_state): State<AppState>,
    Json(request): Json<GenerateEmbeddingRequest>,
) -> Result<Json<GenerateEmbeddingResponse>, (StatusCode, Json<ErrorResponse>)> {
    info!("Generating embedding for text (length: {})", request.text.len());

    match app_state.rag_service.embedding_service.get_embedding(&request.text).await {
        Ok(embedding) => {
            let dimension = embedding.len();
            info!("Successfully generated embedding with dimension: {}", dimension);
            Ok(Json(GenerateEmbeddingResponse {
                embedding,
                dimension,
            }))
        }
        Err(e) => {
            error!("Failed to generate embedding: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: ErrorDetails {
                        code: "EMBEDDING_FAILED".to_string(),
                        message: format!("Failed to generate embedding: {}", e),
                        timestamp: chrono::Utc::now().to_rfc3339(),
                    },
                }),
            ))
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ProcessStoredDocumentRequest {
    pub document_id: i32,
}

#[derive(Debug, Serialize)]
pub struct ProcessStoredDocumentResponse {
    pub success: bool,
    pub document_id: i32,
    pub embedding_generated: bool,
    pub message: String,
}

pub async fn process_stored_document(
    State(app_state): State<AppState>,
    Json(request): Json<ProcessStoredDocumentRequest>,
) -> Result<Json<ProcessStoredDocumentResponse>, (StatusCode, Json<ErrorResponse>)> {
    info!("Processing stored document with ID: {}", request.document_id);

    // First, get the document content from database
    match app_state.rag_service.database.get_document_content(request.document_id).await {
        Ok(Some((filename, content))) => {
            info!("Retrieved document {} with content length: {}", filename, content.len());

            // Generate embedding for the content
            match app_state.rag_service.embedding_service.get_embedding(&content).await {
                Ok(embedding) => {
                    // Store the embedding
                    match app_state.rag_service.database.store_document_embedding(request.document_id, &embedding).await {
                        Ok(_) => {
                            info!("Successfully stored embedding for document {}", request.document_id);
                            Ok(Json(ProcessStoredDocumentResponse {
                                success: true,
                                document_id: request.document_id,
                                embedding_generated: true,
                                message: format!("Successfully generated and stored embedding for document {}", filename),
                            }))
                        }
                        Err(e) => {
                            error!("Failed to store embedding for document {}: {}", request.document_id, e);
                            Err((
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(ErrorResponse {
                                    error: ErrorDetails {
                                        code: "STORAGE_FAILED".to_string(),
                                        message: format!("Failed to store embedding: {}", e),
                                        timestamp: chrono::Utc::now().to_rfc3339(),
                                    },
                                }),
                            ))
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to generate embedding for document {}: {}", request.document_id, e);
                    Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse {
                            error: ErrorDetails {
                                code: "EMBEDDING_FAILED".to_string(),
                                message: format!("Failed to generate embedding: {}", e),
                                timestamp: chrono::Utc::now().to_rfc3339(),
                            },
                        }),
                    ))
                }
            }
        }
        Ok(None) => {
            Err((
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    error: ErrorDetails {
                        code: "DOCUMENT_NOT_FOUND".to_string(),
                        message: format!("Document with ID {} not found", request.document_id),
                        timestamp: chrono::Utc::now().to_rfc3339(),
                    },
                }),
            ))
        }
        Err(e) => {
            error!("Failed to retrieve document {}: {}", request.document_id, e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: ErrorDetails {
                        code: "DATABASE_ERROR".to_string(),
                        message: format!("Failed to retrieve document: {}", e),
                        timestamp: chrono::Utc::now().to_rfc3339(),
                    },
                }),
            ))
        }
    }
}

pub async fn process_document(
    State(app_state): State<AppState>,
    Json(request): Json<ProcessDocumentRequest>,
) -> Result<Json<ProcessDocumentResponse>, (StatusCode, Json<ErrorResponse>)> {
    info!("Processing document: {}", request.filename);

    // Use the RAG service to process the document
    match app_state.rag_service.process_document(&request.filename, &request.content).await {
        Ok(document_id) => {
            info!("Successfully processed document {} with ID {}", request.filename, document_id);
            Ok(Json(ProcessDocumentResponse {
                success: true,
                document_id: Some(document_id),
                chunks_processed: Some(1),
                message: format!("Document {} processed successfully with real AWS Bedrock embeddings", request.filename),
            }))
        }
        Err(e) => {
            error!("Failed to process document {}: {}", request.filename, e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: ErrorDetails {
                        code: "PROCESSING_FAILED".to_string(),
                        message: format!("Failed to process document: {}", e),
                        timestamp: chrono::Utc::now().to_rfc3339(),
                    },
                }),
            ))
        }
    }
}

#[derive(Debug, Serialize)]
pub struct RagModelsResponse {
    pub rag_models: Vec<crate::database::RagModel>,
}

pub async fn get_rag_models(
    State(app_state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<RagModelsResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Extract and validate authentication token
    let _token = extract_and_validate_token(&app_state.auth_client, &headers).await?;

    info!("Fetching available RAG models");

    // For now, we'll return a simple response indicating that this endpoint exists
    // In a full implementation, this would query the UIConfigAPI for RAG models
    Ok(Json(RagModelsResponse {
        rag_models: vec![],
    }))
}