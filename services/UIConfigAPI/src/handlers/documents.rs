use axum::{
    body::Bytes,
    extract::{State, Path, Multipart},
    http::StatusCode,
    Json,
    Extension,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;
use tracing::{error, info, warn};
use crate::{AppState, middleware::AuthUser};

#[derive(Debug, Serialize)]
pub struct FolderInfo {
    pub name: String,
    pub path: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub document_count: usize,
}

#[derive(Debug, Serialize)]
pub struct DocumentInfo {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub content_type: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct FolderListResponse {
    pub folders: Vec<FolderInfo>,
    pub total: usize,
}

#[derive(Debug, Serialize)]
pub struct DocumentListResponse {
    pub documents: Vec<DocumentInfo>,
    pub total: usize,
}

#[derive(Debug, Deserialize)]
pub struct CreateFolderRequest {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct UploadResponse {
    pub success: bool,
    pub message: String,
    pub file_path: Option<String>,
}

pub async fn list_folders(
    State(state): State<AppState>,
    Extension(admin_user): Extension<AuthUser>,
) -> Result<Json<FolderListResponse>, StatusCode> {
    info!("Admin {} requesting document folders list", admin_user.username);

    let documents_path = PathBuf::from(&state.config.documents_base_path);
    
    if !documents_path.exists() {
        info!("Documents directory doesn't exist, creating: {:?}", documents_path);
        if let Err(e) = fs::create_dir_all(&documents_path).await {
            error!("Failed to create documents directory: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    match fs::read_dir(&documents_path).await {
        Ok(mut entries) => {
            let mut folders = Vec::new();
            
            while let Ok(Some(entry)) = entries.next_entry().await {
                if let Ok(metadata) = entry.metadata().await {
                    if metadata.is_dir() {
                        let folder_name = entry.file_name().to_string_lossy().to_string();
                        let folder_path = entry.path();
                        
                        let document_count = match fs::read_dir(&folder_path).await {
                            Ok(mut doc_entries) => {
                                let mut count = 0;
                                while let Ok(Some(_)) = doc_entries.next_entry().await {
                                    count += 1;
                                }
                                count
                            }
                            Err(_) => 0,
                        };

                        let created_at = metadata.created().ok()
                            .and_then(|t| chrono::DateTime::from_timestamp(
                                t.duration_since(std::time::UNIX_EPOCH).ok()?.as_secs() as i64, 0
                            ))
                            .unwrap_or_else(chrono::Utc::now);

                        folders.push(FolderInfo {
                            name: folder_name,
                            path: folder_path.to_string_lossy().to_string(),
                            created_at,
                            document_count,
                        });
                    }
                }
            }

            let total = folders.len();
            Ok(Json(FolderListResponse { folders, total }))
        }
        Err(e) => {
            error!("Failed to read documents directory: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn create_folder(
    State(state): State<AppState>,
    Extension(admin_user): Extension<AuthUser>,
    Json(payload): Json<CreateFolderRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    info!("Admin {} creating document folder: {}", admin_user.username, payload.name);

    if payload.name.is_empty() || payload.name.contains("..") || payload.name.contains("/") || payload.name.contains("\\") {
        return Ok(Json(serde_json::json!({
            "success": false,
            "message": "Invalid folder name"
        })));
    }

    let documents_path = PathBuf::from(&state.config.documents_base_path);
    let folder_path = documents_path.join(&payload.name);

    if folder_path.exists() {
        return Ok(Json(serde_json::json!({
            "success": false,
            "message": "Folder already exists"
        })));
    }

    match fs::create_dir_all(&folder_path).await {
        Ok(()) => {
            info!("Successfully created folder: {}", payload.name);
            
            if let Err(e) = state.db.create_document_folder(&payload.name, &folder_path.to_string_lossy()).await {
                warn!("Failed to record folder in database: {}", e);
            }

            Ok(Json(serde_json::json!({
                "success": true,
                "message": "Folder created successfully",
                "folder_name": payload.name,
                "folder_path": folder_path.to_string_lossy()
            })))
        }
        Err(e) => {
            error!("Failed to create folder {}: {}", payload.name, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn list_documents(
    State(state): State<AppState>,
    Extension(admin_user): Extension<AuthUser>,
    Path(folder_name): Path<String>,
) -> Result<Json<DocumentListResponse>, StatusCode> {
    info!("Admin {} requesting documents in folder: {}", admin_user.username, folder_name);

    let documents_path = PathBuf::from(&state.config.documents_base_path);
    let folder_path = documents_path.join(&folder_name);

    if !folder_path.exists() || !folder_path.is_dir() {
        return Err(StatusCode::NOT_FOUND);
    }

    match fs::read_dir(&folder_path).await {
        Ok(mut entries) => {
            let mut documents = Vec::new();
            
            while let Ok(Some(entry)) = entries.next_entry().await {
                if let Ok(metadata) = entry.metadata().await {
                    if metadata.is_file() {
                        let file_name = entry.file_name().to_string_lossy().to_string();
                        let file_path = entry.path();
                        
                        let created_at = metadata.created().ok()
                            .and_then(|t| chrono::DateTime::from_timestamp(
                                t.duration_since(std::time::UNIX_EPOCH).ok()?.as_secs() as i64, 0
                            ))
                            .unwrap_or_else(chrono::Utc::now);

                        let content_type = mime_guess::from_path(&file_path)
                            .first_or_octet_stream()
                            .to_string();

                        documents.push(DocumentInfo {
                            name: file_name,
                            path: file_path.to_string_lossy().to_string(),
                            size: metadata.len(),
                            created_at,
                            content_type: Some(content_type),
                        });
                    }
                }
            }

            let total = documents.len();
            Ok(Json(DocumentListResponse { documents, total }))
        }
        Err(e) => {
            error!("Failed to read folder {}: {}", folder_name, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn upload_document(
    State(state): State<AppState>,
    Extension(admin_user): Extension<AuthUser>,
    Path(folder_name): Path<String>,
    mut multipart: Multipart,
) -> Result<Json<UploadResponse>, StatusCode> {
    info!("Admin {} uploading document to folder: {}", admin_user.username, folder_name);
    info!("Documents base path: {}", state.config.documents_base_path);
    info!("Max file size: {}", state.config.max_file_size);

    let documents_path = PathBuf::from(&state.config.documents_base_path);
    let folder_path = documents_path.join(&folder_name);

    if !folder_path.exists() {
        return Ok(Json(UploadResponse {
            success: false,
            message: "Folder does not exist".to_string(),
            file_path: None,
        }));
    }

    let mut fields_processed = 0;
    while let Some(field) = multipart.next_field().await.map_err(|e| {
        error!("Failed to read multipart field: {}", e);
        StatusCode::BAD_REQUEST
    })? {
        fields_processed += 1;
        info!("Processing multipart field #{}", fields_processed);
        info!("Field name: {:?}", field.name());
        info!("Field filename: {:?}", field.file_name());
        if let Some(filename) = field.file_name() {
            let filename = filename.to_string();
            
            if filename.is_empty() || filename.contains("..") || filename.contains("/") || filename.contains("\\") {
                return Ok(Json(UploadResponse {
                    success: false,
                    message: "Invalid filename".to_string(),
                    file_path: None,
                }));
            }

            let allowed_extensions = [
                "txt", "md", "pdf", "doc", "docx", "json", "csv", "xml", "html", "rtf"
            ];
            
            let extension = filename.split('.').last().unwrap_or("").to_lowercase();
            if !allowed_extensions.contains(&extension.as_str()) {
                return Ok(Json(UploadResponse {
                    success: false,
                    message: format!("File type not allowed. Allowed types: {}", allowed_extensions.join(", ")),
                    file_path: None,
                }));
            }

            let file_path = folder_path.join(&filename);
            
            if file_path.exists() {
                return Ok(Json(UploadResponse {
                    success: false,
                    message: "File already exists".to_string(),
                    file_path: None,
                }));
            }

            info!("Reading file data for: {}", filename);
            let data = field.bytes().await.map_err(|e| {
                error!("Failed to read file data for {}: {} (error type: {:?})", filename, e, e);
                if e.to_string().contains("body") || e.to_string().contains("length") {
                    error!("This appears to be a content-length or body size issue");
                }
                StatusCode::BAD_REQUEST
            })?;
            
            info!("Successfully read {} bytes for file: {}", data.len(), filename);
            
            if data.len() > state.config.max_file_size {
                return Ok(Json(UploadResponse {
                    success: false,
                    message: format!("File too large. Maximum size: {} bytes", state.config.max_file_size),
                    file_path: None,
                }));
            }

            match fs::write(&file_path, &data).await {
                Ok(()) => {
                    info!("Successfully uploaded file: {} to folder: {}", filename, folder_name);
                    
                    if let Err(e) = state.db.record_document_upload(
                        &folder_name,
                        &filename,
                        &file_path.to_string_lossy(),
                        data.len() as i64,
                        admin_user.id,
                    ).await {
                        warn!("Failed to record document upload in database: {}", e);
                    }

                    return Ok(Json(UploadResponse {
                        success: true,
                        message: "File uploaded successfully".to_string(),
                        file_path: Some(file_path.to_string_lossy().to_string()),
                    }));
                }
                Err(e) => {
                    error!("Failed to save file {}: {}", filename, e);
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
            }
        }
    }

    info!("Total fields processed: {}", fields_processed);
    Ok(Json(UploadResponse {
        success: false,
        message: "No file provided".to_string(),
        file_path: None,
    }))
}

pub async fn delete_document(
    State(state): State<AppState>,
    Extension(admin_user): Extension<AuthUser>,
    Path((folder_name, filename)): Path<(String, String)>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    info!("Admin {} deleting document: {} from folder: {}", admin_user.username, filename, folder_name);

    let documents_path = PathBuf::from(&state.config.documents_base_path);
    let file_path = documents_path.join(&folder_name).join(&filename);

    if !file_path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    match fs::remove_file(&file_path).await {
        Ok(()) => {
            info!("Successfully deleted file: {} from folder: {}", filename, folder_name);
            Ok(Json(serde_json::json!({
                "success": true,
                "message": "Document deleted successfully"
            })))
        }
        Err(e) => {
            error!("Failed to delete file {}: {}", filename, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete_folder(
    State(state): State<AppState>,
    Extension(admin_user): Extension<AuthUser>,
    Path(folder_name): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    info!("Admin {} deleting folder: {}", admin_user.username, folder_name);

    let documents_path = PathBuf::from(&state.config.documents_base_path);
    let folder_path = documents_path.join(&folder_name);

    if !folder_path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    match fs::remove_dir_all(&folder_path).await {
        Ok(()) => {
            info!("Successfully deleted folder: {}", folder_name);
            
            if let Err(e) = state.db.delete_document_folder(&folder_name).await {
                warn!("Failed to remove folder record from database: {}", e);
            }

            Ok(Json(serde_json::json!({
                "success": true,
                "message": "Folder deleted successfully"
            })))
        }
        Err(e) => {
            error!("Failed to delete folder {}: {}", folder_name, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}