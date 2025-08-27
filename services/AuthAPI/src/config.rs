use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub jwt_secret: String,
    pub jwt_expiry_hours: i64,
    pub bcrypt_cost: u32,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenv::dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .map_err(|_| anyhow::anyhow!("DATABASE_URL must be set"))?;

        let host = env::var("HOST")
            .unwrap_or_else(|_| "127.0.0.1".to_string());

        let port = env::var("PORT")
            .unwrap_or_else(|_| "9102".to_string())
            .parse::<u16>()
            .map_err(|e| anyhow::anyhow!("Invalid PORT: {}", e))?;

        let jwt_secret = env::var("JWT_SECRET")
            .unwrap_or_else(|_| "your_super_secret_jwt_key_here_auth_api".to_string());

        let jwt_expiry_hours = env::var("JWT_EXPIRY_HOURS")
            .unwrap_or_else(|_| "24".to_string())
            .parse::<i64>()
            .map_err(|e| anyhow::anyhow!("Invalid JWT_EXPIRY_HOURS: {}", e))?;

        let bcrypt_cost = env::var("BCRYPT_COST")
            .unwrap_or_else(|_| "12".to_string())
            .parse::<u32>()
            .map_err(|e| anyhow::anyhow!("Invalid BCRYPT_COST: {}", e))?;

        Ok(Config {
            database_url,
            host,
            port,
            jwt_secret,
            jwt_expiry_hours,
            bcrypt_cost,
        })
    }
}