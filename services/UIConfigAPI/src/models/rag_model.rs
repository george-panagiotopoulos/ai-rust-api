use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct RagModel {
    pub id: i32,
    pub name: String,
    pub vector_id: i32,
    pub system_prompt: String,
    pub context: Option<String>,
    pub created_by: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct RagModelWithVector {
    pub id: i32,
    pub name: String,
    pub vector_id: i32,
    pub vector_name: String,
    pub system_prompt: String,
    pub context: Option<String>,
    pub created_by: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateRagModelRequest {
    pub name: String,
    pub vector_id: i32,
    pub system_prompt: String,
    pub context: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRagModelRequest {
    pub name: Option<String>,
    pub vector_id: Option<i32>,
    pub system_prompt: Option<String>,
    pub context: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct RagModelListResponse {
    pub rag_models: Vec<RagModelWithVector>,
    pub total: usize,
}

#[derive(Debug, Serialize)]
pub struct RagModelResponse {
    pub success: bool,
    pub message: String,
    pub rag_model: Option<RagModelWithVector>,
}