use crate::{error::AuthError, models::{User, UserSession}};
use chrono::{DateTime, Utc};
use sqlx::PgPool;

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, AuthError> {
        let pool = PgPool::connect(database_url).await?;
        
        // Test the connection
        sqlx::query("SELECT 1").fetch_one(&pool).await?;
        
        Ok(Self { pool })
    }

    pub async fn create_user(
        &self,
        username: &str,
        email: &str,
        password_hash: &str,
    ) -> Result<User, AuthError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (username, email, password_hash, is_active, is_admin)
            VALUES ($1, $2, $3, true, false)
            RETURNING id, username, email, password_hash, is_active, is_admin, created_at, updated_at, last_login
            "#,
        )
        .bind(username)
        .bind(email)
        .bind(password_hash)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            if e.to_string().contains("duplicate key") {
                if e.to_string().contains("username") {
                    AuthError::Conflict("Username already exists".to_string())
                } else if e.to_string().contains("email") {
                    AuthError::Conflict("Email already exists".to_string())
                } else {
                    AuthError::Conflict("User already exists".to_string())
                }
            } else {
                AuthError::Database(e)
            }
        })?;

        Ok(user)
    }

    pub async fn get_user_by_username(&self, username: &str) -> Result<Option<User>, AuthError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, username, email, password_hash, is_active, is_admin, created_at, updated_at, last_login FROM users WHERE username = $1 AND is_active = true"
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn get_user_by_id(&self, user_id: i32) -> Result<Option<User>, AuthError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, username, email, password_hash, is_active, is_admin, created_at, updated_at, last_login FROM users WHERE id = $1 AND is_active = true"
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn update_last_login(&self, user_id: i32) -> Result<(), AuthError> {
        sqlx::query("UPDATE users SET last_login = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP WHERE id = $1")
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn create_session(
        &self,
        user_id: i32,
        token_hash: &str,
        expires_at: DateTime<Utc>,
        user_agent: Option<&str>,
        ip_address: Option<&str>,
    ) -> Result<UserSession, AuthError> {
        let session = sqlx::query_as::<_, UserSession>(
            r#"
            INSERT INTO user_sessions (user_id, token_hash, expires_at, user_agent, ip_address)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, user_id, token_hash, expires_at, created_at, user_agent, ip_address
            "#,
        )
        .bind(user_id)
        .bind(token_hash)
        .bind(expires_at)
        .bind(user_agent)
        .bind(ip_address)
        .fetch_one(&self.pool)
        .await?;

        Ok(session)
    }

    pub async fn get_session_by_token_hash(&self, token_hash: &str) -> Result<Option<UserSession>, AuthError> {
        let session = sqlx::query_as::<_, UserSession>(
            "SELECT id, user_id, token_hash, expires_at, created_at, user_agent, ip_address FROM user_sessions WHERE token_hash = $1 AND expires_at > CURRENT_TIMESTAMP"
        )
        .bind(token_hash)
        .fetch_optional(&self.pool)
        .await?;

        Ok(session)
    }

    pub async fn delete_session(&self, token_hash: &str) -> Result<(), AuthError> {
        sqlx::query("DELETE FROM user_sessions WHERE token_hash = $1")
            .bind(token_hash)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn cleanup_expired_sessions(&self) -> Result<u64, AuthError> {
        let result = sqlx::query("DELETE FROM user_sessions WHERE expires_at <= CURRENT_TIMESTAMP")
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected())
    }

    pub async fn get_user_count(&self) -> Result<i64, AuthError> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users WHERE is_active = true")
            .fetch_one(&self.pool)
            .await?;
        
        Ok(count.0)
    }

    pub async fn get_session_count(&self) -> Result<i64, AuthError> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM user_sessions WHERE expires_at > CURRENT_TIMESTAMP")
            .fetch_one(&self.pool)
            .await?;
        
        Ok(count.0)
    }
}