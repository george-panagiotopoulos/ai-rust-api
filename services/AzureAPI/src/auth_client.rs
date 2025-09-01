use crate::error::AppError;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{error, info};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenValidationRequest {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub is_active: bool,
    pub is_admin: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenValidationResponse {
    pub valid: bool,
    pub user: Option<UserResponse>,
}

#[derive(Clone)]
pub struct AuthClient {
    client: Client,
    auth_api_url: String,
}

impl AuthClient {
    pub fn new(auth_api_url: String) -> Self {
        Self {
            client: Client::new(),
            auth_api_url,
        }
    }

    pub async fn validate_token(&self, token: &str) -> Result<TokenValidationResponse, AppError> {
        let url = format!("{}/validate", self.auth_api_url);
        let request = TokenValidationRequest {
            token: token.to_string(),
        };

        info!("Validating token with AuthAPI at: {}", url);

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                error!("Failed to send validation request: {}", e);
                AppError::AuthError(format!("Failed to validate token: {}", e))
            })?;

        if response.status().is_success() {
            let validation_response: TokenValidationResponse = response.json().await.map_err(|e| {
                error!("Failed to parse validation response: {}", e);
                AppError::AuthError("Invalid response from auth service".to_string())
            })?;

            info!("Token validation result: valid={}", validation_response.valid);
            Ok(validation_response)
        } else {
            let status = response.status();
            error!("Token validation failed with status: {}", status);
            Err(AppError::AuthError(format!(
                "Token validation failed with status: {}",
                status
            )))
        }
    }
}