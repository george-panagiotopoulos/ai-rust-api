pub mod aws_backend;
pub mod azure_backend;
pub mod backend_manager;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use async_trait::async_trait;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LLMRequest {
    pub prompt: String,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LLMResponse {
    pub response: String,
    pub token_count: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmbeddingRequest {
    pub text: String,
    pub model: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbeddingResponse {
    pub embedding: Vec<f32>,
    pub dimension: usize,
}

#[derive(Debug, Clone)]
pub struct BackendConfig {
    pub provider: String,
    pub llm_config: LLMConfig,
    pub embedding_config: EmbeddingConfig,
}

#[derive(Debug, Clone)]
pub struct LLMConfig {
    pub api_key: Option<String>,
    pub endpoint: Option<String>,
    pub model_name: Option<String>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct EmbeddingConfig {
    pub api_key: Option<String>,
    pub endpoint: Option<String>,
    pub model_name: Option<String>,
    pub dimension: Option<u32>,
}

#[async_trait]
pub trait LLMBackend: Send + Sync {
    async fn generate_completion(&self, request: LLMRequest) -> Result<LLMResponse>;
    fn get_name(&self) -> &str;
}

#[async_trait]
pub trait EmbeddingBackend: Send + Sync {
    async fn generate_embedding(&self, request: EmbeddingRequest) -> Result<EmbeddingResponse>;
    fn get_name(&self) -> &str;
}

pub enum BackendType {
    AWS,
    Azure,
}