use axum::{
    extract::{Request, State},
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::Response,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{error, warn};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
    pub client_id: String,
}

#[derive(Clone)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub token_expiry_hours: i64,
}

#[derive(Deserialize)]
pub struct TokenRequest {
    pub grant_type: String,
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub error_description: String,
}

impl AuthConfig {
    pub fn generate_token(&self, client_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
        let now = Utc::now();
        let expiry = now + Duration::hours(self.token_expiry_hours);
        
        let claims = Claims {
            sub: client_id.to_string(),
            exp: expiry.timestamp(),
            iat: now.timestamp(),
            client_id: client_id.to_string(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )
    }

    pub fn validate_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;
        
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_ref()),
            &validation,
        )
        .map(|data| data.claims)
    }

    pub fn validate_client(&self, client_id: &str, client_secret: &str) -> bool {
        // In a real implementation, you would check against a database
        // For demo purposes, we'll use environment variables
        let valid_client_id = std::env::var("OAUTH_CLIENT_ID")
            .unwrap_or_else(|_| "demo_client".to_string());
        let valid_client_secret = std::env::var("OAUTH_CLIENT_SECRET")
            .unwrap_or_else(|_| "demo_secret".to_string());
            
        client_id == valid_client_id && client_secret == valid_client_secret
    }
}

pub async fn auth_middleware(
    State(auth_config): State<Arc<AuthConfig>>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let token = match auth_header {
        Some(auth) if auth.starts_with("Bearer ") => {
            &auth[7..]
        }
        _ => {
            warn!("Missing or invalid Authorization header");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    match auth_config.validate_token(token) {
        Ok(claims) => {
            request.extensions_mut().insert(claims);
            Ok(next.run(request).await)
        }
        Err(e) => {
            error!("Token validation failed: {}", e);
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}