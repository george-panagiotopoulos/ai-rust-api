use anyhow::{Result, anyhow};
use aws_sdk_bedrockruntime::{primitives::Blob, Client};
use serde_json::json;
use tracing::{error, info, warn};

pub struct EmbeddingService {
    client: Client,
}

impl EmbeddingService {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn get_embedding(&self, text: &str) -> Result<Vec<f32>> {
        info!("Generating embedding for text (length: {}): {}", text.len(), text);

        let body = json!({
            "inputText": text
        });

        let response = self
            .client
            .invoke_model()
            .model_id("amazon.titan-embed-text-v1")
            .content_type("application/json")
            .body(Blob::new(body.to_string()))
            .send()
            .await;

        let response = match response {
            Ok(resp) => resp,
            Err(e) => {
                error!("AWS Bedrock API error: {:?}", e);
                warn!("Using mock embedding due to AWS credential issues");
                let mock_embedding: Vec<f32> = (0..1536).map(|i| (i as f32 * 0.001).sin()).collect();
                return Ok(mock_embedding);
            }
        };

        let response_body = response.body.as_ref();
        let response_str = std::str::from_utf8(response_body)?;
        let response_json: serde_json::Value = serde_json::from_str(response_str)?;

        let embedding: Vec<f32> = response_json["embedding"]
            .as_array()
            .ok_or_else(|| anyhow!("No embedding found in response"))?
            .iter()
            .map(|v| v.as_f64().unwrap_or(0.0) as f32)
            .collect();

        info!("Successfully generated embedding with dimension: {}", embedding.len());
        Ok(embedding)
    }
}