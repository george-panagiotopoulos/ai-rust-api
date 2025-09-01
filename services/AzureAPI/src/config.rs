use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub jwt_secret: String,
    pub host: String,
    pub port: u16,
    pub auth_api_url: String,
    pub azure_api_key: String,
    pub azure_endpoint: String,
    pub azure_api_version: String,
    pub azure_llm_deployment: String,
    pub azure_embedding_deployment: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenv::dotenv().ok();

        let jwt_secret = env::var("JWT_SECRET")
            .unwrap_or_else(|_| "default-secret-change-in-production".to_string());

        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("PORT")
            .unwrap_or_else(|_| "9104".to_string())
            .parse()
            .unwrap_or(9104);

        let auth_api_url = env::var("AUTH_API_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:9102".to_string());

        let azure_api_key = env::var("AZURE_OPENAI_API_KEY")
            .map_err(|_| anyhow::anyhow!("AZURE_OPENAI_API_KEY is required"))?;

        let azure_endpoint = env::var("AZURE_OPENAI_ENDPOINT")
            .map_err(|_| anyhow::anyhow!("AZURE_OPENAI_ENDPOINT is required"))?;

        let azure_api_version = env::var("AZURE_OPENAI_API_VERSION")
            .unwrap_or_else(|_| "2024-02-15-preview".to_string());

        let azure_llm_deployment = env::var("AZURE_LLM_DEPLOYMENT_NAME")
            .unwrap_or_else(|_| "gpt-4".to_string());

        let azure_embedding_deployment = env::var("AZURE_EMBEDDING_DEPLOYMENT_NAME")
            .unwrap_or_else(|_| "text-embedding-ada-002".to_string());

        Ok(Config {
            jwt_secret,
            host,
            port,
            auth_api_url,
            azure_api_key,
            azure_endpoint,
            azure_api_version,
            azure_llm_deployment,
            azure_embedding_deployment,
        })
    }

    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}