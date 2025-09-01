use std::collections::HashMap;
use std::path::Path;
use tokio::fs as async_fs;
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigDefinition {
    pub key: String,
    pub description: String,
    pub scope: ConfigScope,
    pub services: Vec<ConfigService>,
    pub required: bool,
    pub sensitive: bool,
    pub default_value: Option<String>,
    pub validation_regex: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConfigScope {
    Global,   // Shared across multiple services - stored in root .env
    Service,  // Service-specific - stored in service .env
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConfigService {
    AuthAPI,
    BedrockAPI,
    RAGAPI,
    UIConfigAPI,
}

impl ConfigService {
    fn get_env_path(&self) -> &'static str {
        match self {
            ConfigService::AuthAPI => "services/AuthAPI/.env",
            ConfigService::BedrockAPI => "services/BedrockAPI/.env",
            ConfigService::RAGAPI => "services/RAGAPI/.env",
            ConfigService::UIConfigAPI => "services/UIConfigAPI/.env",
        }
    }
}

pub struct EnvManager {
    base_path: String,
    global_env_path: String,
    config_definitions: HashMap<String, ConfigDefinition>,
}

impl EnvManager {
    pub fn new(base_path: String) -> Self {
        let mut config_definitions = HashMap::new();
        let global_env_path = format!("{}/.env", base_path);
        
        // Define all possible configurations with their proper scope and services
        let configs = vec![
            // Global configurations (shared across services)
            ConfigDefinition {
                key: "AWS_ACCESS_KEY_ID".to_string(),
                description: "AWS Access Key ID for Bedrock API access".to_string(),
                scope: ConfigScope::Global,
                services: vec![ConfigService::BedrockAPI, ConfigService::RAGAPI],
                required: true,
                sensitive: true,
                default_value: None,
                validation_regex: Some("^AKIA[0-9A-Z]{16}$".to_string()),
            },
            ConfigDefinition {
                key: "AWS_SECRET_ACCESS_KEY".to_string(),
                description: "AWS Secret Access Key for Bedrock API access".to_string(),
                scope: ConfigScope::Global,
                services: vec![ConfigService::BedrockAPI, ConfigService::RAGAPI],
                required: true,
                sensitive: true,
                default_value: None,
                validation_regex: Some("^[A-Za-z0-9/+=]{40}$".to_string()),
            },
            ConfigDefinition {
                key: "AWS_REGION".to_string(),
                description: "AWS Region for Bedrock API".to_string(),
                scope: ConfigScope::Global,
                services: vec![ConfigService::BedrockAPI, ConfigService::RAGAPI],
                required: true,
                sensitive: false,
                default_value: Some("us-east-1".to_string()),
                validation_regex: Some("^[a-z0-9-]+$".to_string()),
            },
            ConfigDefinition {
                key: "DATABASE_URL".to_string(),
                description: "PostgreSQL database connection URL".to_string(),
                scope: ConfigScope::Global,
                services: vec![ConfigService::AuthAPI, ConfigService::RAGAPI, ConfigService::UIConfigAPI],
                required: true,
                sensitive: true,
                default_value: Some("postgresql://raguser:password@localhost:5434/ragdb".to_string()),
                validation_regex: Some("^postgresql://.*$".to_string()),
            },
            ConfigDefinition {
                key: "JWT_SECRET".to_string(),
                description: "JWT signing and validation secret".to_string(),
                scope: ConfigScope::Global,
                services: vec![ConfigService::AuthAPI, ConfigService::BedrockAPI],
                required: true,
                sensitive: true,
                default_value: None,
                validation_regex: Some("^.{32,}$".to_string()),
            },
            ConfigDefinition {
                key: "AZURE_OPENAI_ENDPOINT".to_string(),
                description: "Azure OpenAI service endpoint URL".to_string(),
                scope: ConfigScope::Global,
                services: vec![ConfigService::BedrockAPI, ConfigService::RAGAPI],
                required: true,
                sensitive: false,
                default_value: None,
                validation_regex: Some("^https://.*\\.openai\\.azure\\.com/?$".to_string()),
            },
            ConfigDefinition {
                key: "AZURE_OPENAI_API_KEY".to_string(),
                description: "Azure OpenAI API key for authentication".to_string(),
                scope: ConfigScope::Global,
                services: vec![ConfigService::BedrockAPI, ConfigService::RAGAPI],
                required: true,
                sensitive: true,
                default_value: None,
                validation_regex: Some("^[A-Za-z0-9]+$".to_string()),
            },
            ConfigDefinition {
                key: "AZURE_OPENAI_API_VERSION".to_string(),
                description: "Azure OpenAI API version".to_string(),
                scope: ConfigScope::Global,
                services: vec![ConfigService::BedrockAPI, ConfigService::RAGAPI],
                required: true,
                sensitive: false,
                default_value: Some("2024-05-01-preview".to_string()),
                validation_regex: Some("^[0-9]{4}-[0-9]{2}-[0-9]{2}(-preview)?$".to_string()),
            },
            ConfigDefinition {
                key: "AZURE_OPENAI_DEPLOYMENT".to_string(),
                description: "Azure OpenAI deployment/model name for chat completions".to_string(),
                scope: ConfigScope::Global,
                services: vec![ConfigService::BedrockAPI, ConfigService::RAGAPI],
                required: true,
                sensitive: false,
                default_value: None,
                validation_regex: Some("^[a-zA-Z0-9-]+$".to_string()),
            },
            ConfigDefinition {
                key: "AZURE_OPENAI_EMBEDDING_MODEL".to_string(),
                description: "Azure OpenAI embedding model name".to_string(),
                scope: ConfigScope::Global,
                services: vec![ConfigService::BedrockAPI, ConfigService::RAGAPI],
                required: true,
                sensitive: false,
                default_value: Some("text-embedding-3-large".to_string()),
                validation_regex: Some("^[a-zA-Z0-9-]+$".to_string()),
            },
            
            // AuthAPI service-specific configurations
            ConfigDefinition {
                key: "HOST".to_string(),
                description: "AuthAPI server host address".to_string(),
                scope: ConfigScope::Service,
                services: vec![ConfigService::AuthAPI],
                required: false,
                sensitive: false,
                default_value: Some("127.0.0.1".to_string()),
                validation_regex: Some("^[0-9.]+$".to_string()),
            },
            ConfigDefinition {
                key: "PORT".to_string(),
                description: "AuthAPI server port number".to_string(),
                scope: ConfigScope::Service,
                services: vec![ConfigService::AuthAPI],
                required: false,
                sensitive: false,
                default_value: Some("9102".to_string()),
                validation_regex: Some("^[0-9]+$".to_string()),
            },
            ConfigDefinition {
                key: "JWT_EXPIRY_HOURS".to_string(),
                description: "JWT token expiration time in hours".to_string(),
                scope: ConfigScope::Service,
                services: vec![ConfigService::AuthAPI],
                required: false,
                sensitive: false,
                default_value: Some("48".to_string()),
                validation_regex: Some("^[0-9]+$".to_string()),
            },
            ConfigDefinition {
                key: "BCRYPT_COST".to_string(),
                description: "BCrypt hashing cost (rounds)".to_string(),
                scope: ConfigScope::Service,
                services: vec![ConfigService::AuthAPI],
                required: false,
                sensitive: false,
                default_value: Some("12".to_string()),
                validation_regex: Some("^[0-9]+$".to_string()),
            },
            
            // BedrockAPI service-specific configurations
            ConfigDefinition {
                key: "HOST".to_string(),
                description: "BedrockAPI server host address".to_string(),
                scope: ConfigScope::Service,
                services: vec![ConfigService::BedrockAPI],
                required: false,
                sensitive: false,
                default_value: Some("127.0.0.1".to_string()),
                validation_regex: Some("^[0-9.]+$".to_string()),
            },
            ConfigDefinition {
                key: "PORT".to_string(),
                description: "BedrockAPI server port number".to_string(),
                scope: ConfigScope::Service,
                services: vec![ConfigService::BedrockAPI],
                required: false,
                sensitive: false,
                default_value: Some("9100".to_string()),
                validation_regex: Some("^[0-9]+$".to_string()),
            },
            
            // RAGAPI service-specific configurations
            ConfigDefinition {
                key: "HOST".to_string(),
                description: "RAGAPI server host address".to_string(),
                scope: ConfigScope::Service,
                services: vec![ConfigService::RAGAPI],
                required: false,
                sensitive: false,
                default_value: Some("127.0.0.1".to_string()),
                validation_regex: Some("^[0-9.]+$".to_string()),
            },
            ConfigDefinition {
                key: "PORT".to_string(),
                description: "RAGAPI server port number".to_string(),
                scope: ConfigScope::Service,
                services: vec![ConfigService::RAGAPI],
                required: false,
                sensitive: false,
                default_value: Some("9101".to_string()),
                validation_regex: Some("^[0-9]+$".to_string()),
            },
            ConfigDefinition {
                key: "BEDROCK_API_URL".to_string(),
                description: "Bedrock API service URL".to_string(),
                scope: ConfigScope::Service,
                services: vec![ConfigService::RAGAPI],
                required: true,
                sensitive: false,
                default_value: Some("http://127.0.0.1:9100".to_string()),
                validation_regex: Some("^https?://.*$".to_string()),
            },
            ConfigDefinition {
                key: "EMBEDDING_MODEL".to_string(),
                description: "AWS Bedrock embedding model".to_string(),
                scope: ConfigScope::Service,
                services: vec![ConfigService::RAGAPI],
                required: false,
                sensitive: false,
                default_value: Some("amazon.titan-embed-text-v1".to_string()),
                validation_regex: Some("^[a-zA-Z0-9.-]+$".to_string()),
            },
            ConfigDefinition {
                key: "EMBEDDING_DIMENSION".to_string(),
                description: "Dimension size for embeddings".to_string(),
                scope: ConfigScope::Service,
                services: vec![ConfigService::RAGAPI],
                required: false,
                sensitive: false,
                default_value: Some("1536".to_string()),
                validation_regex: Some("^[0-9]+$".to_string()),
            },
            
            // UIConfigAPI service-specific configurations
            ConfigDefinition {
                key: "HOST".to_string(),
                description: "UIConfigAPI server host address".to_string(),
                scope: ConfigScope::Service,
                services: vec![ConfigService::UIConfigAPI],
                required: false,
                sensitive: false,
                default_value: Some("127.0.0.1".to_string()),
                validation_regex: Some("^[0-9.]+$".to_string()),
            },
            ConfigDefinition {
                key: "PORT".to_string(),
                description: "UIConfigAPI server port number".to_string(),
                scope: ConfigScope::Service,
                services: vec![ConfigService::UIConfigAPI],
                required: false,
                sensitive: false,
                default_value: Some("9103".to_string()),
                validation_regex: Some("^[0-9]+$".to_string()),
            },
            ConfigDefinition {
                key: "AUTH_API_URL".to_string(),
                description: "Authentication API service URL".to_string(),
                scope: ConfigScope::Service,
                services: vec![ConfigService::UIConfigAPI],
                required: true,
                sensitive: false,
                default_value: Some("http://127.0.0.1:9102".to_string()),
                validation_regex: Some("^https?://.*$".to_string()),
            },
            ConfigDefinition {
                key: "BEDROCK_API_URL".to_string(),
                description: "Bedrock API service URL".to_string(),
                scope: ConfigScope::Service,
                services: vec![ConfigService::UIConfigAPI],
                required: true,
                sensitive: false,
                default_value: Some("http://127.0.0.1:9100".to_string()),
                validation_regex: Some("^https?://.*$".to_string()),
            },
            ConfigDefinition {
                key: "RAGAPI_URL".to_string(),
                description: "RAG API service URL".to_string(),
                scope: ConfigScope::Service,
                services: vec![ConfigService::UIConfigAPI],
                required: true,
                sensitive: false,
                default_value: Some("http://127.0.0.1:9101".to_string()),
                validation_regex: Some("^https?://.*$".to_string()),
            },
            ConfigDefinition {
                key: "DOCUMENTS_BASE_PATH".to_string(),
                description: "Base path for document storage".to_string(),
                scope: ConfigScope::Service,
                services: vec![ConfigService::UIConfigAPI],
                required: true,
                sensitive: false,
                default_value: Some("/Users/gpanagiotopoulos/ai-rust-api/services/RAGAPI/documents".to_string()),
                validation_regex: Some("^[/\\w.-]+$".to_string()),
            },
            ConfigDefinition {
                key: "MAX_UPLOAD_SIZE".to_string(),
                description: "Maximum file upload size in bytes".to_string(),
                scope: ConfigScope::Service,
                services: vec![ConfigService::UIConfigAPI],
                required: false,
                sensitive: false,
                default_value: Some("10485760".to_string()),
                validation_regex: Some("^[0-9]+$".to_string()),
            },
            ConfigDefinition {
                key: "ENCRYPTION_KEY".to_string(),
                description: "Encryption key for sensitive configuration data".to_string(),
                scope: ConfigScope::Service,
                services: vec![ConfigService::UIConfigAPI],
                required: true,
                sensitive: true,
                default_value: None,
                validation_regex: Some("^[A-Za-z0-9+/=]+$".to_string()),
            },
            ConfigDefinition {
                key: "ALLOWED_EXTENSIONS".to_string(),
                description: "Allowed file extensions for upload".to_string(),
                scope: ConfigScope::Service,
                services: vec![ConfigService::UIConfigAPI],
                required: false,
                sensitive: false,
                default_value: Some("pdf,txt,md,docx".to_string()),
                validation_regex: Some("^[a-zA-Z0-9,]+$".to_string()),
            },
        ];

        for config in configs {
            config_definitions.insert(config.key.clone(), config);
        }

        Self {
            base_path,
            global_env_path,
            config_definitions,
        }
    }

    pub fn get_config_definitions(&self) -> &HashMap<String, ConfigDefinition> {
        &self.config_definitions
    }

    pub async fn update_config(&self, key: &str, value: &str) -> Result<Vec<String>> {
        let config_def = self.config_definitions.get(key)
            .ok_or_else(|| anyhow!("Unknown configuration key: {}", key))?;

        // Validate the value
        if let Some(regex) = &config_def.validation_regex {
            let re = regex::Regex::new(regex)?;
            if !re.is_match(value) {
                return Err(anyhow!("Invalid value format for key: {}", key));
            }
        }

        let mut updated_files = Vec::new();

        match config_def.scope {
            ConfigScope::Global => {
                // Update the global .env file
                self.update_env_file(&Path::new(&self.global_env_path), key, value).await?;
                updated_files.push(".env".to_string());
            }
            ConfigScope::Service => {
                // Update service-specific .env files
                for service in &config_def.services {
                    let service_env_path = Path::new(&self.base_path).join(service.get_env_path());
                    if let Err(e) = self.update_env_file(&service_env_path, key, value).await {
                        warn!("Failed to update {}: {}", service.get_env_path(), e);
                    } else {
                        updated_files.push(service.get_env_path().to_string());
                    }
                }
            }
        }

        info!("Updated configuration {} in files: {:?}", key, updated_files);
        Ok(updated_files)
    }

    async fn update_env_file(&self, file_path: &Path, key: &str, value: &str) -> Result<()> {
        // Create parent directory if it doesn't exist
        if let Some(parent) = file_path.parent() {
            async_fs::create_dir_all(parent).await?;
        }

        let content = if file_path.exists() {
            async_fs::read_to_string(file_path).await?
        } else {
            String::new()
        };

        let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        let mut found = false;

        // Update existing key or add new one
        for line in lines.iter_mut() {
            if line.starts_with(&format!("{}=", key)) {
                *line = format!("{}={}", key, value);
                found = true;
                break;
            }
        }

        if !found {
            lines.push(format!("{}={}", key, value));
        }

        let new_content = lines.join("\n") + "\n";
        async_fs::write(file_path, new_content).await?;

        Ok(())
    }

    pub async fn get_current_value(&self, key: &str, service: &ConfigService) -> Result<Option<String>> {
        let config_def = self.config_definitions.get(key)
            .ok_or_else(|| anyhow!("Unknown configuration key: {}", key))?;

        let file_path = match config_def.scope {
            ConfigScope::Global => Path::new(&self.global_env_path).to_path_buf(),
            ConfigScope::Service => {
                if config_def.services.contains(service) {
                    Path::new(&self.base_path).join(service.get_env_path())
                } else {
                    return Ok(None);
                }
            }
        };

        if file_path.exists() {
            let content = async_fs::read_to_string(&file_path).await?;
            for line in content.lines() {
                if line.starts_with(&format!("{}=", key)) {
                    if let Some(value) = line.split('=').nth(1) {
                        return Ok(Some(value.to_string()));
                    }
                }
            }
        }

        Ok(config_def.default_value.clone())
    }

    pub async fn validate_all_configs(&self) -> Result<Vec<String>> {
        let mut errors = Vec::new();

        for (key, config_def) in &self.config_definitions {
            if config_def.required {
                for service in &config_def.services {
                    match self.get_current_value(key, service).await {
                        Ok(Some(_)) => {}, // Value exists
                        Ok(None) => {
                            errors.push(format!("Missing required configuration: {} for service: {:?}", key, service));
                        }
                        Err(e) => {
                            errors.push(format!("Error reading configuration {}: {}", key, e));
                        }
                    }
                }
            }
        }

        Ok(errors)
    }
}