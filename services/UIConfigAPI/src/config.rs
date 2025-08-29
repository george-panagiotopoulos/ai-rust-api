use anyhow::{anyhow, Result};
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub auth_api_url: String,
    pub bedrock_api_url: String,
    pub rag_api_url: String,
    pub documents_base_path: String,
    pub max_file_size: usize,
    pub allowed_extensions: Vec<String>,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        // Load global .env file first
        if let Ok(root_env) = std::path::Path::new("../../.env").canonicalize() {
            dotenv::from_path(root_env).ok();
        }
        // Then load service-specific .env file (if it exists)
        dotenv::dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .map_err(|_| anyhow!("DATABASE_URL must be set"))?;
        
        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        
        let port = env::var("PORT")
            .unwrap_or_else(|_| "9103".to_string())
            .parse::<u16>()
            .map_err(|_| anyhow!("Invalid PORT value"))?;

        let auth_api_url = env::var("AUTH_API_URL")
            .map_err(|_| anyhow!("AUTH_API_URL must be set"))?;
        
        let bedrock_api_url = env::var("BEDROCK_API_URL")
            .map_err(|_| anyhow!("BEDROCK_API_URL must be set"))?;
        
        let rag_api_url = env::var("RAGAPI_URL")
            .map_err(|_| anyhow!("RAGAPI_URL must be set"))?;
        
        let documents_base_path = env::var("DOCUMENTS_BASE_PATH")
            .map_err(|_| anyhow!("DOCUMENTS_BASE_PATH must be set"))?;
        
        let max_file_size = env::var("MAX_UPLOAD_SIZE")
            .unwrap_or_else(|_| "10485760".to_string()) // 10MB default
            .parse::<usize>()
            .map_err(|_| anyhow!("Invalid MAX_UPLOAD_SIZE value"))?;
        
        let allowed_extensions = env::var("ALLOWED_EXTENSIONS")
            .unwrap_or_else(|_| "pdf,txt,md,docx".to_string())
            .split(',')
            .map(|s| s.trim().to_lowercase())
            .collect();

        Ok(Config {
            database_url,
            host,
            port,
            auth_api_url,
            bedrock_api_url,
            rag_api_url,
            documents_base_path,
            max_file_size,
            allowed_extensions,
        })
    }

    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}