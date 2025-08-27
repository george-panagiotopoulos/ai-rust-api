use crate::{error::AuthError, models::User};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // subject (user_id)
    pub username: String,
    pub is_admin: bool,
    pub exp: i64,     // expiration
    pub iat: i64,     // issued at
}

pub struct JwtManager {
    secret: String,
    expiry_hours: i64,
}

impl JwtManager {
    pub fn new(secret: String, expiry_hours: i64) -> Self {
        Self {
            secret,
            expiry_hours,
        }
    }

    pub fn create_token(&self, user: &User) -> Result<(String, chrono::DateTime<Utc>), AuthError> {
        let now = Utc::now();
        let expires_at = now + Duration::hours(self.expiry_hours);

        let claims = Claims {
            sub: user.id.to_string(),
            username: user.username.clone(),
            is_admin: user.is_admin,
            exp: expires_at.timestamp(),
            iat: now.timestamp(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )?;

        Ok((token, expires_at))
    }

    pub fn validate_token(&self, token: &str) -> Result<Claims, AuthError> {
        let mut validation = Validation::default();
        validation.required_spec_claims = HashSet::new(); // Don't require any specific claims

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &validation,
        )?;

        Ok(token_data.claims)
    }

    pub fn hash_password(&self, password: &str, cost: u32) -> Result<String, AuthError> {
        bcrypt::hash(password, cost).map_err(AuthError::BCrypt)
    }

    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool, AuthError> {
        bcrypt::verify(password, hash).map_err(AuthError::BCrypt)
    }
}