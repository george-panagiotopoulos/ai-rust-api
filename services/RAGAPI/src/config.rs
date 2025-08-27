use anyhow::Result;
use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub aws_region: String,
    pub bedrock_api_url: String,
    pub auth_api_url: String,
    pub embedding_model: String,
    pub embedding_dimension: usize,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenv::dotenv().ok();

        Ok(Config {
            host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8081".to_string())
                .parse()?,
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://raguser:password@localhost:5432/ragdb".to_string()),
            aws_region: env::var("AWS_REGION").unwrap_or_else(|_| "us-east-1".to_string()),
            bedrock_api_url: env::var("BEDROCK_API_URL")
                .unwrap_or_else(|_| "http://127.0.0.1:8080".to_string()),
            auth_api_url: env::var("AUTH_API_URL")
                .unwrap_or_else(|_| "http://127.0.0.1:9102".to_string()),
            embedding_model: env::var("EMBEDDING_MODEL")
                .unwrap_or_else(|_| "amazon.titan-embed-text-v1".to_string()),
            embedding_dimension: env::var("EMBEDDING_DIMENSION")
                .unwrap_or_else(|_| "1536".to_string())
                .parse()?,
        })
    }

    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}