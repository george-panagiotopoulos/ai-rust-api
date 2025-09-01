use super::{LLMBackend, EmbeddingBackend, LLMRequest, LLMResponse, EmbeddingRequest, EmbeddingResponse, BackendConfig};
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use serde_json::{json, Value};
use reqwest::Client;
use tracing::{error, info};

pub struct AzureLLMBackend {
    client: Client,
    config: BackendConfig,
}

pub struct AzureEmbeddingBackend {
    client: Client,
    config: BackendConfig,
}

impl AzureLLMBackend {
    pub fn new(config: BackendConfig) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }
}

impl AzureEmbeddingBackend {
    pub fn new(config: BackendConfig) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }
}

#[async_trait]
impl LLMBackend for AzureLLMBackend {
    async fn generate_completion(&self, request: LLMRequest) -> Result<LLMResponse> {
        info!("Generating completion using Azure OpenAI");
        
        // Use the AzureAPI service
        if let Some(endpoint) = &self.config.llm_config.endpoint {
            let azure_request = json!({
                "prompt": request.prompt,
                "max_tokens": request.max_tokens.unwrap_or(4096),
                "temperature": request.temperature.unwrap_or(0.7),
                "top_p": request.top_p.unwrap_or(1.0)
            });

            let response = self.client
                .post(&format!("{}/chat", endpoint))
                .header("Content-Type", "application/json")
                .json(&azure_request)
                .send()
                .await
                .map_err(|e| anyhow!("Failed to send request to AzureAPI: {}", e))?;

            if response.status().is_success() {
                let response_data: Value = response.json().await
                    .map_err(|e| anyhow!("Failed to parse AzureAPI response: {}", e))?;
                
                Ok(LLMResponse {
                    response: response_data["response"].as_str().unwrap_or("").to_string(),
                    token_count: response_data["token_count"].as_u64().map(|t| t as u32),
                })
            } else {
                let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                Err(anyhow!("AzureAPI request failed: {}", error_text))
            }
        } else {
            Err(anyhow!("Azure endpoint not configured"))
        }
    }

    fn get_name(&self) -> &str {
        "Azure OpenAI"
    }
}

#[async_trait]
impl EmbeddingBackend for AzureEmbeddingBackend {
    async fn generate_embedding(&self, request: EmbeddingRequest) -> Result<EmbeddingResponse> {
        info!("Generating embedding using Azure OpenAI");
        
        // Use the AzureAPI service
        if let Some(endpoint) = &self.config.embedding_config.endpoint {
            let azure_request = json!({
                "input": request.text,
                "model": request.model.as_ref().unwrap_or(&self.config.embedding_config.model_name.as_ref().unwrap_or(&"text-embedding-ada-002".to_string()).clone())
            });

            let response = self.client
                .post(&format!("{}/embeddings", endpoint))
                .header("Content-Type", "application/json")
                .json(&azure_request)
                .send()
                .await
                .map_err(|e| anyhow!("Failed to send request to AzureAPI: {}", e))?;

            if response.status().is_success() {
                let response_data: Value = response.json().await
                    .map_err(|e| anyhow!("Failed to parse AzureAPI response: {}", e))?;
                
                if let Some(embeddings_array) = response_data["embeddings"].as_array() {
                    if let Some(first_embedding) = embeddings_array.first() {
                        if let Some(embedding_data) = first_embedding["embedding"].as_array() {
                            let embedding: Result<Vec<f32>, _> = embedding_data
                                .iter()
                                .map(|v| v.as_f64().map(|f| f as f32).ok_or_else(|| anyhow!("Invalid embedding value")))
                                .collect();
                            
                            let embedding = embedding?;
                            let dimension = embedding.len();
                            
                            return Ok(EmbeddingResponse {
                                embedding,
                                dimension,
                            });
                        }
                    }
                }
                
                Err(anyhow!("Invalid embedding response format"))
            } else {
                let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                Err(anyhow!("AzureAPI request failed: {}", error_text))
            }
        } else {
            Err(anyhow!("Azure endpoint not configured"))
        }
    }

    fn get_name(&self) -> &str {
        "Azure OpenAI Embeddings"
    }
}