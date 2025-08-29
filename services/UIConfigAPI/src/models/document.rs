use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateFolderRequest {
    #[validate(length(min = 1, max = 255))]
    pub folder_name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct FolderResponse {
    pub id: i32,
    pub folder_name: String,
    pub description: Option<String>,
    pub file_count: usize,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct DocumentInfo {
    pub name: String,
    pub size: u64,
    pub extension: String,
    pub modified: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct UploadResponse {
    pub success: bool,
    pub filename: String,
    pub folder: String,
    pub size: u64,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct FolderContentsResponse {
    pub folder_name: String,
    pub documents: Vec<DocumentInfo>,
    pub total_size: u64,
}