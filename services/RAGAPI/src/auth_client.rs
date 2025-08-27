use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::error;

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

    pub async fn validate_token(&self, token: &str) -> Result<TokenValidationResponse, reqwest::Error> {
        let request = TokenValidationRequest {
            token: token.to_string(),
        };

        let response = self
            .client
            .post(&format!("{}/validate", self.auth_api_url))
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let validation_response: TokenValidationResponse = response
                .json()
                .await?;
            
            Ok(validation_response)
        } else {
            error!("Token validation failed with status: {}", response.status());
            Ok(TokenValidationResponse {
                valid: false,
                user: None,
            })
        }
    }
}