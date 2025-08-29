use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct ConfigUpdateRequest {
    pub settings: HashMap<String, ServiceConfig>,
}

#[derive(Debug, Deserialize)]
pub struct ServiceConfig {
    pub settings: HashMap<String, ConfigValue>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigValue {
    pub value: String,
    pub is_sensitive: bool,
}

#[derive(Debug, Serialize)]
pub struct ConfigResponse {
    pub services: HashMap<String, HashMap<String, ConfigValueResponse>>,
}

#[derive(Debug, Serialize)]
pub struct ConfigValueResponse {
    pub value: String,
    pub is_encrypted: bool,
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_by: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct ConfigApplyResponse {
    pub success: bool,
    pub updated_files: Vec<String>,
    pub errors: Vec<String>,
}