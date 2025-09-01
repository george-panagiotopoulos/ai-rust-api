use super::{LLMBackend, EmbeddingBackend, LLMRequest, LLMResponse, EmbeddingRequest, EmbeddingResponse, BackendConfig};
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use serde_json::{json, Value};
use reqwest::Client;
use tracing::{error, info};

pub struct AWSLLMBackend {
    client: Client,
    config: BackendConfig,
}

pub struct AWSEmbeddingBackend {
    client: Client,
    config: BackendConfig,
}

impl AWSLLMBackend {
    pub fn new(config: BackendConfig) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }
}

impl AWSEmbeddingBackend {
    pub fn new(config: BackendConfig) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }
}

#[async_trait]
impl LLMBackend for AWSLLMBackend {
    async fn generate_completion(&self, request: LLMRequest) -> Result<LLMResponse> {
        info!("Generating completion using AWS Bedrock");
        
        // Use the existing BedrockAPI service
        if let Some(endpoint) = &self.config.llm_config.endpoint {
            let bedrock_request = json!({
                "prompt": request.prompt,
                "max_tokens": request.max_tokens.unwrap_or(4096),
                "temperature": request.temperature.unwrap_or(0.7),
                "top_p": request.top_p.unwrap_or(1.0)
            });

            let response = self.client
                .post(&format!("{}/chat", endpoint))
                .header("Content-Type", "application/json")
                .json(&bedrock_request)
                .send()
                .await
                .map_err(|e| anyhow!("Failed to send request to BedrockAPI: {}", e))?;

            if response.status().is_success() {
                let response_data: Value = response.json().await
                    .map_err(|e| anyhow!("Failed to parse BedrockAPI response: {}", e))?;
                
                Ok(LLMResponse {
                    response: response_data["response"].as_str().unwrap_or("").to_string(),
                    token_count: response_data["token_count"].as_u64().map(|t| t as u32),
                })
            } else {
                let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                Err(anyhow!("BedrockAPI request failed: {}", error_text))
            }
        } else {
            Err(anyhow!("AWS endpoint not configured"))
        }
    }

    fn get_name(&self) -> &str {
        "AWS Bedrock"
    }
}

#[async_trait]
impl EmbeddingBackend for AWSEmbeddingBackend {
    async fn generate_embedding(&self, request: EmbeddingRequest) -> Result<EmbeddingResponse> {
        info!("Generating embedding using AWS Bedrock");
        
        // Mock implementation - in real scenario, this would use AWS Bedrock embeddings
        // For now, return a fixed-dimension mock embedding
        let dimension = self.config.embedding_config.dimension.unwrap_or(1536) as usize;
        let mock_embedding: Vec<f32> = (0..dimension)
            .map(|i| (i as f32 * 0.001) % 1.0)
            .collect();

        Ok(EmbeddingResponse {
            embedding: mock_embedding,
            dimension,
        })
    }

    fn get_name(&self) -> &str {
        "AWS Bedrock Embeddings"
    }
}