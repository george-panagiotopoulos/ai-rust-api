use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use tracing::info;
use crate::utils::EncryptionManager;

pub struct Database {
    pool: PgPool,
    encryption: EncryptionManager,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = PgPool::connect(database_url).await?;
        sqlx::query("SELECT 1").fetch_one(&pool).await?;
        
        let encryption = EncryptionManager::from_env()?;
        
        Self::init_ui_tables(&pool).await?;
        
        Ok(Self { pool, encryption })
    }

    async fn init_ui_tables(pool: &PgPool) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS config_settings (
                key VARCHAR(100) PRIMARY KEY,
                value TEXT NOT NULL,
                is_encrypted BOOLEAN DEFAULT false,
                description TEXT,
                created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS document_folders (
                id SERIAL PRIMARY KEY,
                folder_name VARCHAR(255) NOT NULL UNIQUE,
                folder_path TEXT NOT NULL,
                created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS chat_history (
                id SERIAL PRIMARY KEY,
                user_id INTEGER NOT NULL,
                conversation_id VARCHAR(255) NOT NULL,
                user_message TEXT NOT NULL,
                assistant_response TEXT NOT NULL,
                created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(pool)
        .await?;

        // Create vectors table for RAG vector management
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS vectors (
                id SERIAL PRIMARY KEY,
                name VARCHAR(255) NOT NULL,
                folder_name VARCHAR(255) NOT NULL,
                description TEXT,
                document_count INTEGER DEFAULT 0,
                embedding_count INTEGER DEFAULT 0,
                created_by INTEGER,
                created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
                is_active BOOLEAN DEFAULT true
            )
            "#,
        )
        .execute(pool)
        .await?;

        // Create RAG models table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS rag_models (
                id SERIAL PRIMARY KEY,
                name VARCHAR(255) NOT NULL,
                vector_id INTEGER REFERENCES vectors(id),
                system_prompt TEXT NOT NULL,
                context TEXT,
                created_by INTEGER,
                created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
                is_active BOOLEAN DEFAULT true
            )
            "#,
        )
        .execute(pool)
        .await?;

        info!("UI tables initialized successfully");
        Ok(())
    }

    // User management methods
    pub async fn create_user(&self, username: &str, email: &str, password: &str, is_admin: bool) -> Result<i32> {
        let hashed_password = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;
        let row = sqlx::query!(
            "INSERT INTO users (username, email, password_hash, is_admin, is_active) VALUES ($1, $2, $3, $4, true) RETURNING id",
            username, email, hashed_password, is_admin
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(row.id)
    }

    pub async fn get_user_by_id(&self, user_id: i32) -> Result<Option<crate::models::User>> {
        let row = sqlx::query_as!(
            crate::models::User,
            "SELECT id, username, email, is_active, is_admin, created_at, updated_at FROM users WHERE id = $1",
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn list_all_users(&self) -> Result<Vec<crate::models::User>> {
        let users = sqlx::query_as!(
            crate::models::User,
            "SELECT id, username, email, is_active, is_admin, created_at, updated_at FROM users ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(users)
    }

    pub async fn update_user_status(&self, user_id: i32, is_active: bool) -> Result<()> {
        sqlx::query!(
            "UPDATE users SET is_active = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2",
            is_active, user_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn update_user_admin_status(&self, user_id: i32, is_admin: bool) -> Result<()> {
        sqlx::query!(
            "UPDATE users SET is_admin = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2",
            is_admin, user_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn delete_user(&self, user_id: i32) -> Result<bool> {
        let result = sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    // Config management methods
    pub async fn get_config_setting(&self, key: &str) -> Result<Option<crate::models::ConfigSetting>> {
        let row = sqlx::query_as!(
            crate::models::ConfigSetting,
            "SELECT key, value, is_encrypted, description, created_at, updated_at FROM config_settings WHERE key = $1",
            key
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn list_config_settings(&self) -> Result<Vec<crate::models::ConfigSetting>> {
        let configs = sqlx::query_as!(
            crate::models::ConfigSetting,
            "SELECT key, value, is_encrypted, description, created_at, updated_at FROM config_settings ORDER BY key"
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(configs)
    }

    pub async fn create_config_setting(&self, key: &str, value: &str, is_encrypted: bool, description: Option<&str>) -> Result<()> {
        sqlx::query!(
            "INSERT INTO config_settings (key, value, is_encrypted, description) VALUES ($1, $2, $3, $4)",
            key, value, is_encrypted, description
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn update_config_setting(&self, key: &str, value: &str) -> Result<()> {
        sqlx::query!(
            "UPDATE config_settings SET value = $1, updated_at = CURRENT_TIMESTAMP WHERE key = $2",
            value, key
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn delete_config_setting(&self, key: &str) -> Result<bool> {
        let result = sqlx::query!("DELETE FROM config_settings WHERE key = $1", key)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    // Encryption methods
    pub async fn encrypt_value(&self, value: &str) -> Result<String> {
        self.encryption.encrypt(value)
    }

    pub async fn decrypt_value(&self, encrypted_value: &str) -> Result<String> {
        self.encryption.decrypt(encrypted_value)
    }

    // Chat history methods
    pub async fn save_chat_history(&self, user_id: i32, conversation_id: &str, user_message: &str, assistant_response: &str) -> Result<()> {
        sqlx::query!(
            "INSERT INTO chat_history (user_id, conversation_id, user_message, assistant_response) VALUES ($1, $2, $3, $4)",
            user_id, conversation_id, user_message, assistant_response
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_chat_history_by_user(&self, user_id: i32) -> Result<Vec<crate::models::ChatHistory>> {
        let history = sqlx::query_as!(
            crate::models::ChatHistory,
            "SELECT id, user_id, conversation_id, user_message, assistant_response, created_at FROM chat_history WHERE user_id = $1 ORDER BY created_at DESC LIMIT 100",
            user_id
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(history)
    }

    // Document folder methods
    pub async fn create_document_folder(&self, folder_name: &str, folder_path: &str) -> Result<()> {
        sqlx::query!(
            "INSERT INTO document_folders (folder_name, folder_path) VALUES ($1, $2)",
            folder_name, folder_path
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn delete_document_folder(&self, folder_name: &str) -> Result<()> {
        sqlx::query!("DELETE FROM document_folders WHERE folder_name = $1", folder_name)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn record_document_upload(&self, _folder_name: &str, _filename: &str, _file_path: &str, _file_size: i64, _uploaded_by: i32) -> Result<()> {
        // For now, just return Ok. Could add a documents table later if needed
        Ok(())
    }

    // Admin overview methods
    pub async fn get_user_count(&self) -> Result<i64> {
        let result = sqlx::query!("SELECT COUNT(*) as count FROM users")
            .fetch_one(&self.pool)
            .await?;
        Ok(result.count.unwrap_or(0))
    }

    pub async fn get_active_user_count(&self) -> Result<i64> {
        let result = sqlx::query!("SELECT COUNT(*) as count FROM users WHERE is_active = true")
            .fetch_one(&self.pool)
            .await?;
        Ok(result.count.unwrap_or(0))
    }

    pub async fn get_admin_user_count(&self) -> Result<i64> {
        let result = sqlx::query!("SELECT COUNT(*) as count FROM users WHERE is_admin = true AND is_active = true")
            .fetch_one(&self.pool)
            .await?;
        Ok(result.count.unwrap_or(0))
    }

    pub async fn get_recent_login_count(&self) -> Result<i64> {
        let result = sqlx::query!("SELECT COUNT(*) as count FROM users WHERE last_login > NOW() - INTERVAL '24 hours'")
            .fetch_one(&self.pool)
            .await?;
        Ok(result.count.unwrap_or(0))
    }

    pub async fn get_document_count(&self) -> Result<i64> {
        let result = sqlx::query!("SELECT COUNT(*) as count FROM documents")
            .fetch_one(&self.pool)
            .await?;
        Ok(result.count.unwrap_or(0))
    }

    pub async fn test_connection(&self) -> bool {
        sqlx::query("SELECT 1").fetch_one(&self.pool).await.is_ok()
    }

    pub async fn get_recent_activity(&self, limit: i32) -> Result<Vec<crate::handlers::ActivityLog>> {
        // This would require an activity log table. For now, return empty vec
        // Could be implemented with a proper activity tracking table
        let _limit = limit; // Use the parameter to avoid warnings
        Ok(vec![])
    }

    // Vector management methods
    pub async fn create_vector(&self, name: &str, folder_name: &str, description: Option<&str>, created_by: Option<i32>) -> Result<i32> {
        let row = sqlx::query!(
            "INSERT INTO vectors (name, folder_name, description, created_by) VALUES ($1, $2, $3, $4) RETURNING id",
            name, folder_name, description, created_by
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(row.id)
    }

    pub async fn get_vector_by_id(&self, vector_id: i32) -> Result<Option<crate::models::Vector>> {
        let row = sqlx::query_as!(
            crate::models::Vector,
            "SELECT id, name, folder_name, description, document_count, embedding_count, created_by, created_at, updated_at, is_active FROM vectors WHERE id = $1",
            vector_id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn list_vectors(&self) -> Result<Vec<crate::models::Vector>> {
        let vectors = sqlx::query_as!(
            crate::models::Vector,
            "SELECT id, name, folder_name, description, document_count, embedding_count, created_by, created_at, updated_at, is_active FROM vectors WHERE is_active = true ORDER BY updated_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(vectors)
    }

    pub async fn update_vector_counts(&self, vector_id: i32, document_count: i32, embedding_count: i32) -> Result<()> {
        sqlx::query!(
            "UPDATE vectors SET document_count = $1, embedding_count = $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3",
            document_count, embedding_count, vector_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn delete_vector(&self, vector_id: i32) -> Result<()> {
        sqlx::query!("UPDATE vectors SET is_active = false WHERE id = $1", vector_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    // RAG Model management methods
    pub async fn create_rag_model(&self, name: &str, vector_id: i32, system_prompt: &str, context: Option<&str>, created_by: Option<i32>) -> Result<i32> {
        let row = sqlx::query!(
            "INSERT INTO rag_models (name, vector_id, system_prompt, context, created_by) VALUES ($1, $2, $3, $4, $5) RETURNING id",
            name, vector_id, system_prompt, context, created_by
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(row.id)
    }

    pub async fn get_rag_model_by_id(&self, model_id: i32) -> Result<Option<crate::models::RagModelWithVector>> {
        let row = sqlx::query_as!(
            crate::models::RagModelWithVector,
            r#"
            SELECT r.id, r.name, r.vector_id, v.name as vector_name, r.system_prompt, r.context, r.created_by, r.created_at, r.updated_at, r.is_active 
            FROM rag_models r 
            JOIN vectors v ON r.vector_id = v.id 
            WHERE r.id = $1
            "#,
            model_id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn list_rag_models(&self) -> Result<Vec<crate::models::RagModelWithVector>> {
        let models = sqlx::query_as!(
            crate::models::RagModelWithVector,
            r#"
            SELECT r.id, r.name, r.vector_id, v.name as vector_name, r.system_prompt, r.context, r.created_by, r.created_at, r.updated_at, r.is_active 
            FROM rag_models r 
            JOIN vectors v ON r.vector_id = v.id 
            WHERE r.is_active = true 
            ORDER BY r.updated_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(models)
    }

    pub async fn update_rag_model(&self, model_id: i32, name: Option<&str>, vector_id: Option<i32>, system_prompt: Option<&str>, context: Option<&str>, is_active: Option<bool>) -> Result<()> {
        if let Some(name) = name {
            sqlx::query!(
                "UPDATE rag_models SET name = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2",
                name, model_id
            )
            .execute(&self.pool)
            .await?;
        }

        if let Some(vector_id) = vector_id {
            sqlx::query!(
                "UPDATE rag_models SET vector_id = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2",
                vector_id, model_id
            )
            .execute(&self.pool)
            .await?;
        }

        if let Some(system_prompt) = system_prompt {
            sqlx::query!(
                "UPDATE rag_models SET system_prompt = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2",
                system_prompt, model_id
            )
            .execute(&self.pool)
            .await?;
        }

        if let Some(context) = context {
            sqlx::query!(
                "UPDATE rag_models SET context = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2",
                context, model_id
            )
            .execute(&self.pool)
            .await?;
        }

        if let Some(is_active) = is_active {
            sqlx::query!(
                "UPDATE rag_models SET is_active = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2",
                is_active, model_id
            )
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }

    pub async fn delete_rag_model(&self, model_id: i32) -> Result<()> {
        sqlx::query!("UPDATE rag_models SET is_active = false WHERE id = $1", model_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}