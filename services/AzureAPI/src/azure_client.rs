use crate::config::Config;
use crate::error::AppError;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{error, info};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatRequest {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleChatRequest {
    pub prompt: String,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f64>,
    pub top_p: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatResponse {
    pub id: String,
    pub response: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleChatResponse {
    pub response: String,
    pub token_count: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AzureChatRequest {
    pub messages: Vec<ChatMessage>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f64>,
    pub top_p: Option<f64>,
    pub stream: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AzureChatChoice {
    pub message: ChatMessage,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AzureChatUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AzureChatResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub choices: Vec<AzureChatChoice>,
    pub usage: Option<AzureChatUsage>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AzureEmbeddingRequest {
    pub input: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AzureEmbeddingData {
    pub embedding: Vec<f32>,
    pub index: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AzureEmbeddingResponse {
    pub data: Vec<AzureEmbeddingData>,
    pub usage: Option<AzureEmbeddingUsage>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AzureEmbeddingUsage {
    pub prompt_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Clone)]
pub struct AzureClient {
    client: Client,
    config: Config,
}

impl AzureClient {
    pub fn new(config: Config) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }

    pub async fn chat_completion(&self, request: ChatRequest) -> Result<ChatResponse, AppError> {
        let messages = vec![ChatMessage {
            role: "user".to_string(),
            content: request.message.clone(),
        }];

        let azure_request = AzureChatRequest {
            messages,
            max_tokens: Some(1000),
            temperature: Some(0.7),
            top_p: Some(0.9),
            stream: false,
        };

        let url = format!(
            "{}/openai/deployments/{}/chat/completions?api-version={}",
            self.config.azure_endpoint, self.config.azure_llm_deployment, self.config.azure_api_version
        );

        info!("Sending chat completion request to Azure: {}", url);

        let response = self
            .client
            .post(&url)
            .header("api-key", &self.config.azure_api_key)
            .header("Content-Type", "application/json")
            .json(&azure_request)
            .send()
            .await
            .map_err(|e| {
                error!("Failed to send chat completion request: {}", e);
                AppError::AzureError(format!("Failed to send request: {}", e))
            })?;

        if response.status().is_success() {
            let azure_response: AzureChatResponse = response.json().await.map_err(|e| {
                error!("Failed to parse Azure response: {}", e);
                AppError::AzureError("Invalid response from Azure OpenAI".to_string())
            })?;

            let response_text = azure_response
                .choices
                .first()
                .map(|choice| choice.message.content.clone())
                .unwrap_or_else(|| "No response generated".to_string());

            info!("Successfully received chat completion response");

            Ok(ChatResponse {
                id: Uuid::new_v4().to_string(),
                response: response_text,
            })
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            error!("Azure OpenAI request failed: {} - {}", status, error_text);
            Err(AppError::AzureError(format!(
                "Azure OpenAI request failed: {} - {}",
                status, error_text
            )))
        }
    }

    pub async fn simple_chat(&self, request: SimpleChatRequest) -> Result<SimpleChatResponse, AppError> {
        let messages = vec![ChatMessage {
            role: "user".to_string(),
            content: request.prompt.clone(),
        }];

        let azure_request = AzureChatRequest {
            messages,
            max_tokens: request.max_tokens,
            temperature: request.temperature,
            top_p: request.top_p,
            stream: false,
        };

        let url = format!(
            "{}/openai/deployments/{}/chat/completions?api-version={}",
            self.config.azure_endpoint, self.config.azure_llm_deployment, self.config.azure_api_version
        );

        info!("Sending simple chat request to Azure: {}", url);

        let response = self
            .client
            .post(&url)
            .header("api-key", &self.config.azure_api_key)
            .header("Content-Type", "application/json")
            .json(&azure_request)
            .send()
            .await
            .map_err(|e| {
                error!("Failed to send simple chat request: {}", e);
                AppError::AzureError(format!("Failed to send request: {}", e))
            })?;

        if response.status().is_success() {
            let azure_response: AzureChatResponse = response.json().await.map_err(|e| {
                error!("Failed to parse Azure response: {}", e);
                AppError::AzureError("Invalid response from Azure OpenAI".to_string())
            })?;

            let response_text = azure_response
                .choices
                .first()
                .map(|choice| choice.message.content.clone())
                .unwrap_or_else(|| "No response generated".to_string());

            let token_count = azure_response
                .usage
                .map(|usage| usage.total_tokens);

            info!("Successfully received simple chat response");

            Ok(SimpleChatResponse {
                response: response_text,
                token_count,
            })
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            error!("Azure OpenAI simple chat failed: {} - {}", status, error_text);
            Err(AppError::AzureError(format!(
                "Azure OpenAI simple chat failed: {} - {}",
                status, error_text
            )))
        }
    }

    pub async fn create_embedding(&self, text: &str) -> Result<Vec<f32>, AppError> {
        let request = AzureEmbeddingRequest {
            input: text.to_string(),
        };

        let url = format!(
            "{}/openai/deployments/{}/embeddings?api-version={}",
            self.config.azure_endpoint, self.config.azure_embedding_deployment, self.config.azure_api_version
        );

        info!("Sending embedding request to Azure: {} (text length: {})", url, text.len());

        let response = self
            .client
            .post(&url)
            .header("api-key", &self.config.azure_api_key)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                error!("Failed to send embedding request: {}", e);
                AppError::AzureError(format!("Failed to send embedding request: {}", e))
            })?;

        if response.status().is_success() {
            let azure_response: AzureEmbeddingResponse = response.json().await.map_err(|e| {
                error!("Failed to parse Azure embedding response: {}", e);
                AppError::AzureError("Invalid embedding response from Azure OpenAI".to_string())
            })?;

            let embedding = azure_response
                .data
                .first()
                .map(|data| data.embedding.clone())
                .ok_or_else(|| {
                    error!("No embedding data in Azure response");
                    AppError::AzureError("No embedding data in response".to_string())
                })?;

            info!("Successfully received embedding (dimension: {})", embedding.len());
            Ok(embedding)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            error!("Azure OpenAI embedding failed: {} - {}", status, error_text);
            Err(AppError::AzureError(format!(
                "Azure OpenAI embedding failed: {} - {}",
                status, error_text
            )))
        }
    }
}