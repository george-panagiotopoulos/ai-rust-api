use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateAdminUserRequest {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserPermissionsRequest {
    pub is_admin: bool,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub is_active: bool,
    pub is_admin: bool,
}

#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct ChatResponse {
    pub response: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}