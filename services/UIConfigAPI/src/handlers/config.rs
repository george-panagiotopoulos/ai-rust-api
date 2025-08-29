use axum::{extract::{State, Path}, http::StatusCode, Json, Extension};
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};
use crate::{AppState, middleware::AuthUser, models::ConfigSetting, env_manager::{ConfigService, ConfigDefinition, ConfigScope}};

#[derive(Debug, Deserialize)]
pub struct UpdateConfigRequest {
    pub value: String,
}

#[derive(Debug, Serialize)]
pub struct ConfigResponse {
    pub key: String,
    pub value: String,
    pub is_encrypted: bool,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ConfigListResponse {
    pub configs: Vec<ConfigResponse>,
    pub total: usize,
}

#[derive(Debug, Deserialize)]
pub struct CreateConfigRequest {
    pub key: String,
    pub value: String,
    pub is_encrypted: bool,
    pub description: Option<String>,
}

pub async fn list_configs(
    State(state): State<AppState>,
    Extension(admin_user): Extension<AuthUser>,
) -> Result<Json<ConfigListResponse>, StatusCode> {
    info!("Admin {} requesting configuration list", admin_user.username);

    match state.db.list_config_settings().await {
        Ok(configs) => {
            let config_responses: Vec<ConfigResponse> = configs
                .into_iter()
                .map(|config| ConfigResponse {
                    key: config.key,
                    value: if config.is_encrypted.unwrap_or(false) {
                        "********".to_string()
                    } else {
                        config.value
                    },
                    is_encrypted: config.is_encrypted.unwrap_or(false),
                    description: config.description,
                })
                .collect();
            
            let total = config_responses.len();
            Ok(Json(ConfigListResponse {
                configs: config_responses,
                total,
            }))
        }
        Err(e) => {
            error!("Failed to list configurations: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_config(
    State(state): State<AppState>,
    Extension(admin_user): Extension<AuthUser>,
    Path(key): Path<String>,
) -> Result<Json<ConfigResponse>, StatusCode> {
    info!("Admin {} requesting configuration for key: {}", admin_user.username, key);

    match state.db.get_config_setting(&key).await {
        Ok(Some(config)) => {
            let response = ConfigResponse {
                key: config.key,
                value: if config.is_encrypted.unwrap_or(false) {
                    match state.db.decrypt_value(&config.value).await {
                        Ok(decrypted) => decrypted,
                        Err(e) => {
                            error!("Failed to decrypt config value: {}", e);
                            return Err(StatusCode::INTERNAL_SERVER_ERROR);
                        }
                    }
                } else {
                    config.value
                },
                is_encrypted: config.is_encrypted.unwrap_or(false),
                description: config.description,
            };
            Ok(Json(response))
        }
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            error!("Failed to get configuration {}: {}", key, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn create_config(
    State(state): State<AppState>,
    Extension(admin_user): Extension<AuthUser>,
    Json(payload): Json<CreateConfigRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    info!("Admin {} creating configuration: {}", admin_user.username, payload.key);

    let stored_value = if payload.is_encrypted {
        match state.db.encrypt_value(&payload.value).await {
            Ok(encrypted) => encrypted,
            Err(e) => {
                error!("Failed to encrypt config value: {}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
    } else {
        payload.value
    };

    match state.db.create_config_setting(
        &payload.key,
        &stored_value,
        payload.is_encrypted,
        payload.description.as_deref(),
    ).await {
        Ok(()) => {
            info!("Successfully created configuration: {}", payload.key);
            Ok(Json(serde_json::json!({
                "success": true,
                "message": "Configuration created successfully",
                "key": payload.key
            })))
        }
        Err(e) => {
            error!("Failed to create configuration {}: {}", payload.key, e);
            if e.to_string().contains("unique constraint") {
                Ok(Json(serde_json::json!({
                    "success": false,
                    "message": "Configuration key already exists"
                })))
            } else {
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}

pub async fn update_config(
    State(state): State<AppState>,
    Extension(admin_user): Extension<AuthUser>,
    Path(key): Path<String>,
    Json(payload): Json<UpdateConfigRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    info!("Admin {} updating configuration: {}", admin_user.username, key);

    let existing_config = match state.db.get_config_setting(&key).await {
        Ok(Some(config)) => config,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(e) => {
            error!("Failed to get existing configuration {}: {}", key, e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let stored_value = if existing_config.is_encrypted.unwrap_or(false) {
        match state.db.encrypt_value(&payload.value).await {
            Ok(encrypted) => encrypted,
            Err(e) => {
                error!("Failed to encrypt config value: {}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
    } else {
        payload.value
    };

    match state.db.update_config_setting(&key, &stored_value).await {
        Ok(()) => {
            info!("Successfully updated configuration: {}", key);
            Ok(Json(serde_json::json!({
                "success": true,
                "message": "Configuration updated successfully"
            })))
        }
        Err(e) => {
            error!("Failed to update configuration {}: {}", key, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete_config(
    State(state): State<AppState>,
    Extension(admin_user): Extension<AuthUser>,
    Path(key): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    info!("Admin {} deleting configuration: {}", admin_user.username, key);

    match state.db.delete_config_setting(&key).await {
        Ok(true) => {
            info!("Successfully deleted configuration: {}", key);
            Ok(Json(serde_json::json!({
                "success": true,
                "message": "Configuration deleted successfully"
            })))
        }
        Ok(false) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            error!("Failed to delete configuration {}: {}", key, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn backup_configs(
    State(state): State<AppState>,
    Extension(admin_user): Extension<AuthUser>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    info!("Admin {} requesting configuration backup", admin_user.username);

    match state.db.list_config_settings().await {
        Ok(configs) => {
            let mut backup_data = Vec::new();
            
            for config in configs {
                let value = if config.is_encrypted.unwrap_or(false) {
                    match state.db.decrypt_value(&config.value).await {
                        Ok(decrypted) => decrypted,
                        Err(e) => {
                            warn!("Failed to decrypt config {} for backup: {}", config.key, e);
                            continue;
                        }
                    }
                } else {
                    config.value
                };

                backup_data.push(serde_json::json!({
                    "key": config.key,
                    "value": value,
                    "is_encrypted": config.is_encrypted.unwrap_or(false),
                    "description": config.description,
                    "created_at": config.created_at,
                    "updated_at": config.updated_at
                }));
            }

            Ok(Json(serde_json::json!({
                "backup_data": backup_data,
                "backup_timestamp": chrono::Utc::now(),
                "total_configs": backup_data.len()
            })))
        }
        Err(e) => {
            error!("Failed to create configuration backup: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// New .env configuration management endpoints

#[derive(Debug, Serialize)]
pub struct EnvConfigDefinitionResponse {
    pub key: String,
    pub description: String,
    pub scope: String,
    pub services: Vec<String>,
    pub required: bool,
    pub sensitive: bool,
    pub default_value: Option<String>,
    pub current_value: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct EnvConfigListResponse {
    pub configs: Vec<EnvConfigDefinitionResponse>,
    pub total: usize,
}

#[derive(Debug, Deserialize)]
pub struct UpdateEnvConfigRequest {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Serialize)]
pub struct ValidationResponse {
    pub valid: bool,
    pub errors: Vec<String>,
}

pub async fn list_env_configs(
    State(state): State<AppState>,
    Extension(admin_user): Extension<AuthUser>,
) -> Result<Json<EnvConfigListResponse>, StatusCode> {
    info!("Admin {} requesting .env configuration definitions", admin_user.username);

    let mut configs = Vec::new();
    
    for (key, config_def) in state.env_manager.get_config_definitions() {
        let scope_name = match config_def.scope {
            ConfigScope::Global => "Global".to_string(),
            ConfigScope::Service => "Service".to_string(),
        };

        let service_names: Vec<String> = config_def.services.iter().map(|s| {
            match s {
                ConfigService::AuthAPI => "AuthAPI".to_string(),
                ConfigService::BedrockAPI => "BedrockAPI".to_string(),
                ConfigService::RAGAPI => "RAGAPI".to_string(),
                ConfigService::UIConfigAPI => "UIConfigAPI".to_string(),
            }
        }).collect();

        // Get current value from the first applicable service
        let current_value = if !config_def.services.is_empty() {
            state.env_manager.get_current_value(key, &config_def.services[0]).await.ok().flatten()
        } else {
            None
        };

        configs.push(EnvConfigDefinitionResponse {
            key: key.clone(),
            description: config_def.description.clone(),
            scope: scope_name,
            services: service_names,
            required: config_def.required,
            sensitive: config_def.sensitive,
            default_value: config_def.default_value.clone(),
            current_value: if config_def.sensitive && current_value.is_some() {
                Some("********".to_string())
            } else {
                current_value
            },
        });
    }

    configs.sort_by(|a, b| a.scope.cmp(&b.scope).then(a.key.cmp(&b.key)));
    let total = configs.len();

    Ok(Json(EnvConfigListResponse { configs, total }))
}

pub async fn update_env_config(
    State(state): State<AppState>,
    Extension(admin_user): Extension<AuthUser>,
    Json(payload): Json<UpdateEnvConfigRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    info!("Admin {} updating .env configuration: {}", admin_user.username, payload.key);

    match state.env_manager.update_config(&payload.key, &payload.value).await {
        Ok(updated_files) => {
            info!("Successfully updated configuration {} in files: {:?}", payload.key, updated_files);
            Ok(Json(serde_json::json!({
                "success": true,
                "message": "Configuration updated successfully",
                "updated_files": updated_files
            })))
        }
        Err(e) => {
            error!("Failed to update configuration {}: {}", payload.key, e);
            Ok(Json(serde_json::json!({
                "success": false,
                "message": e.to_string()
            })))
        }
    }
}

pub async fn get_env_config(
    State(state): State<AppState>,
    Extension(admin_user): Extension<AuthUser>,
    Path(key): Path<String>,
) -> Result<Json<EnvConfigDefinitionResponse>, StatusCode> {
    info!("Admin {} requesting .env configuration for key: {}", admin_user.username, key);

    if let Some(config_def) = state.env_manager.get_config_definitions().get(&key) {
        let scope_name = match config_def.scope {
            ConfigScope::Global => "Global".to_string(),
            ConfigScope::Service => "Service".to_string(),
        };

        let service_names: Vec<String> = config_def.services.iter().map(|s| {
            match s {
                ConfigService::AuthAPI => "AuthAPI".to_string(),
                ConfigService::BedrockAPI => "BedrockAPI".to_string(),
                ConfigService::RAGAPI => "RAGAPI".to_string(),
                ConfigService::UIConfigAPI => "UIConfigAPI".to_string(),
            }
        }).collect();

        // Get current value from the first applicable service
        let current_value = if !config_def.services.is_empty() {
            state.env_manager.get_current_value(&key, &config_def.services[0]).await.ok().flatten()
        } else {
            None
        };

        let response = EnvConfigDefinitionResponse {
            key: key.clone(),
            description: config_def.description.clone(),
            scope: scope_name,
            services: service_names,
            required: config_def.required,
            sensitive: config_def.sensitive,
            default_value: config_def.default_value.clone(),
            current_value: if !config_def.sensitive {
                current_value
            } else {
                current_value.map(|_| "********".to_string())
            },
        };

        Ok(Json(response))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn validate_env_configs(
    State(state): State<AppState>,
    Extension(admin_user): Extension<AuthUser>,
) -> Result<Json<ValidationResponse>, StatusCode> {
    info!("Admin {} requesting .env configuration validation", admin_user.username);

    match state.env_manager.validate_all_configs().await {
        Ok(errors) => {
            let valid = errors.is_empty();
            Ok(Json(ValidationResponse { valid, errors }))
        }
        Err(e) => {
            error!("Failed to validate configurations: {}", e);
            Ok(Json(ValidationResponse {
                valid: false,
                errors: vec![e.to_string()],
            }))
        }
    }
}