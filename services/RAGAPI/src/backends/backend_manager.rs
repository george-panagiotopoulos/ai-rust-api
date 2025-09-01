use super::{
    LLMBackend, EmbeddingBackend, LLMRequest, LLMResponse, EmbeddingRequest, EmbeddingResponse, 
    BackendConfig, BackendType,
    aws_backend::{AWSLLMBackend, AWSEmbeddingBackend},
    azure_backend::{AzureLLMBackend, AzureEmbeddingBackend}
};
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use tracing::{info, warn};
use std::sync::Arc;

pub struct BackendManager {
    llm_backend: Box<dyn LLMBackend>,
    embedding_backend: Box<dyn EmbeddingBackend>,
    active_backend: String,
}

impl BackendManager {
    pub async fn new(ui_config_url: &str) -> Result<Self> {
        // Fetch active backend configuration from UIConfigAPI
        let backend_config = Self::fetch_backend_config(ui_config_url).await?;
        
        let (llm_backend, embedding_backend) = match backend_config.provider.as_str() {
            "aws" => {
                info!("Initializing AWS backend");
                let llm: Box<dyn LLMBackend> = Box::new(AWSLLMBackend::new(backend_config.clone()));
                let embedding: Box<dyn EmbeddingBackend> = Box::new(AWSEmbeddingBackend::new(backend_config.clone()));
                (llm, embedding)
            }
            "azure" => {
                info!("Initializing Azure backend");
                let llm: Box<dyn LLMBackend> = Box::new(AzureLLMBackend::new(backend_config.clone()));
                let embedding: Box<dyn EmbeddingBackend> = Box::new(AzureEmbeddingBackend::new(backend_config.clone()));
                (llm, embedding)
            }
            provider => {
                return Err(anyhow!("Unsupported backend provider: {}", provider));
            }
        };

        Ok(Self {
            llm_backend,
            embedding_backend,
            active_backend: backend_config.provider,
        })
    }

    async fn fetch_backend_config(ui_config_url: &str) -> Result<BackendConfig> {
        let client = reqwest::Client::new();
        
        // Get active backend status
        let status_response = client
            .get(&format!("{}/admin/backends/status", ui_config_url))
            .send()
            .await
            .map_err(|e| anyhow!("Failed to fetch backend status: {}", e))?;

        if !status_response.status().is_success() {
            return Err(anyhow!("Failed to fetch backend status: HTTP {}", status_response.status()));
        }

        let status_data: serde_json::Value = status_response.json().await
            .map_err(|e| anyhow!("Failed to parse backend status: {}", e))?;

        let active_backend = status_data["active_backend"]
            .as_str()
            .ok_or_else(|| anyhow!("No active backend found"))?;

        if active_backend == "none" {
            return Err(anyhow!("No backend is currently active"));
        }

        // Get backend configurations
        let config_response = client
            .get(&format!("{}/admin/backends", ui_config_url))
            .send()
            .await
            .map_err(|e| anyhow!("Failed to fetch backend configs: {}", e))?;

        if !config_response.status().is_success() {
            return Err(anyhow!("Failed to fetch backend configs: HTTP {}", config_response.status()));
        }

        let config_data: serde_json::Value = config_response.json().await
            .map_err(|e| anyhow!("Failed to parse backend configs: {}", e))?;

        let backends = config_data["backends"]
            .as_array()
            .ok_or_else(|| anyhow!("Invalid backend config format"))?;

        // Find the active backend configuration
        for backend in backends {
            if backend["provider"].as_str() == Some(active_backend) && backend["is_active"].as_bool() == Some(true) {
                return Self::parse_backend_config(backend);
            }
        }

        Err(anyhow!("Active backend configuration not found"))
    }

    fn parse_backend_config(backend_data: &serde_json::Value) -> Result<BackendConfig> {
        let provider = backend_data["provider"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing provider"))?
            .to_string();

        let llm_config = super::LLMConfig {
            api_key: backend_data["llm_config"]["api_key"].as_str().map(|s| s.to_string()),
            endpoint: backend_data["llm_config"]["endpoint"].as_str().map(|s| s.to_string()),
            model_name: backend_data["llm_config"]["model_name"].as_str().map(|s| s.to_string()),
            max_tokens: backend_data["llm_config"]["max_tokens"].as_u64().map(|t| t as u32),
            temperature: backend_data["llm_config"]["temperature"].as_f64(),
        };

        let embedding_config = super::EmbeddingConfig {
            api_key: backend_data["embedding_config"]["api_key"].as_str().map(|s| s.to_string()),
            endpoint: backend_data["embedding_config"]["endpoint"].as_str().map(|s| s.to_string()),
            model_name: backend_data["embedding_config"]["model_name"].as_str().map(|s| s.to_string()),
            dimension: backend_data["embedding_config"]["dimension"].as_u64().map(|d| d as u32),
        };

        Ok(BackendConfig {
            provider,
            llm_config,
            embedding_config,
        })
    }

    pub async fn generate_completion(&self, request: LLMRequest) -> Result<LLMResponse> {
        info!("Using {} for completion", self.llm_backend.get_name());
        self.llm_backend.generate_completion(request).await
    }

    pub async fn generate_embedding(&self, request: EmbeddingRequest) -> Result<EmbeddingResponse> {
        info!("Using {} for embedding", self.embedding_backend.get_name());
        self.embedding_backend.generate_embedding(request).await
    }

    pub fn get_active_backend(&self) -> &str {
        &self.active_backend
    }

    pub async fn refresh_config(&mut self, ui_config_url: &str) -> Result<()> {
        info!("Refreshing backend configuration");
        let new_manager = Self::new(ui_config_url).await?;
        self.llm_backend = new_manager.llm_backend;
        self.embedding_backend = new_manager.embedding_backend;
        self.active_backend = new_manager.active_backend;
        info!("Backend configuration refreshed, now using: {}", self.active_backend);
        Ok(())
    }
}