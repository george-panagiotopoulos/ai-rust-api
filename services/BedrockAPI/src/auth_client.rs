use crate::error::AppError;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct TokenValidationRequest {
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub is_active: bool,
    pub is_admin: bool,
}

#[derive(Debug, Deserialize)]
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
        let request = TokenValidationRequest {
            token: token.to_string(),
        };

        let response = self
            .client
            .post(&format!("{}/validate", self.auth_api_url))
            .json(&request)
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to validate token with AuthAPI: {}", e)))?;

        if response.status().is_success() {
            let validation_response: TokenValidationResponse = response
                .json()
                .await
                .map_err(|e| AppError::Internal(format!("Failed to parse validation response: {}", e)))?;
            
            Ok(validation_response)
        } else {
            Err(AppError::Unauthorized("Invalid token".to_string()))
        }
    }
}