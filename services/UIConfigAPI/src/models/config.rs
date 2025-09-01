use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct ConfigUpdateRequest {
    pub settings: HashMap<String, ServiceConfig>,
}

#[derive(Debug, Deserialize)]
pub struct ServiceConfig {
    pub settings: HashMap<String, ConfigValue>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigValue {
    pub value: String,
    pub is_sensitive: bool,
}

#[derive(Debug, Serialize)]
pub struct ConfigResponse {
    pub services: HashMap<String, HashMap<String, ConfigValueResponse>>,
}

#[derive(Debug, Serialize)]
pub struct ConfigValueResponse {
    pub value: String,
    pub is_encrypted: bool,
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_by: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct ConfigApplyResponse {
    pub success: bool,
    pub updated_files: Vec<String>,
    pub errors: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BackendConfig {
    pub id: Option<i32>,
    pub provider: String, // "aws" | "azure"
    pub is_active: bool,
    pub llm_config: LLMConfig,
    pub embedding_config: EmbeddingConfig,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LLMConfig {
    pub api_key: Option<String>,
    pub endpoint: Option<String>,
    pub model_name: Option<String>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmbeddingConfig {
    pub api_key: Option<String>,
    pub endpoint: Option<String>,
    pub model_name: Option<String>,
    pub dimension: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BackendConfigRequest {
    pub provider: String,
    pub is_active: bool,
    pub llm_config: LLMConfig,
    pub embedding_config: EmbeddingConfig,
}

#[derive(Debug, Serialize)]
pub struct BackendConfigResponse {
    pub backends: Vec<BackendConfig>,
    pub active_backend: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct BackendStatusResponse {
    pub aws_status: String,
    pub azure_status: String,
    pub active_backend: String,
}