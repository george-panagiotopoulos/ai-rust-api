pub mod vector;
pub mod rag_model;
pub mod config;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub use vector::*;
pub use rag_model::*;
pub use config::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub is_active: Option<bool>,
    pub is_admin: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigSetting {
    pub key: String,
    pub value: String,
    pub is_encrypted: Option<bool>,
    pub description: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatHistory {
    pub id: i32,
    pub user_id: i32,
    pub conversation_id: String,
    pub user_message: String,
    pub assistant_response: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentFolder {
    pub id: i32,
    pub folder_name: String,
    pub folder_path: String,
    pub created_at: Option<DateTime<Utc>>,
}