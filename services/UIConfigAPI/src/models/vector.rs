use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Vector {
    pub id: i32,
    pub name: String,
    pub folder_name: String,
    pub description: Option<String>,
    pub document_count: Option<i32>,
    pub embedding_count: Option<i32>,
    pub created_by: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateVectorRequest {
    pub name: String,
    pub folder_name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct VectorListResponse {
    pub vectors: Vec<Vector>,
    pub total: usize,
}

#[derive(Debug, Serialize)]
pub struct VectorCreationResponse {
    pub success: bool,
    pub message: String,
    pub vector_id: Option<i32>,
    pub processing_status: String,
}