use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{error, info};

#[derive(Debug, Serialize, Deserialize)]
pub struct BedrockRequest {
    pub prompt: String,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BedrockResponse {
    pub response: String,
    pub token_count: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RAGRequest {
    pub query: String,
    pub system_prompt: Option<String>,
    pub context: Option<String>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub rag_model_name: Option<String>,
}

pub struct BedrockApiClient {
    client: Client,
    base_url: String,
}

impl BedrockApiClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    pub async fn generate_response(
        &self,
        prompt: &str,
        max_tokens: Option<u32>,
        temperature: Option<f32>,
        token: &str,
    ) -> Result<BedrockResponse> {
        let request = BedrockRequest {
            prompt: prompt.to_string(),
            max_tokens,
            temperature,
            top_p: None,
        };

        info!("Sending request to BedrockAPI: {}", self.base_url);

        let response = self
            .client
            .post(&format!("{}/simple-chat", self.base_url))
            .header("Authorization", format!("Bearer {}", token))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            error!("BedrockAPI error {}: {}", status, error_text);
            return Err(anyhow::anyhow!(
                "BedrockAPI request failed with status {}: {}",
                status,
                error_text
            ));
        }

        let bedrock_response: BedrockResponse = response.json().await?;
        Ok(bedrock_response)
    }
}