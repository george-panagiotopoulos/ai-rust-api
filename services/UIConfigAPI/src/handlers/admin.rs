use axum::{extract::{State, Path}, http::StatusCode, Json, Extension};
use serde::{Deserialize, Serialize};
use tracing::{error, info};
use std::time::{SystemTime, UNIX_EPOCH, Instant};
use std::sync::OnceLock;
use crate::{AppState, middleware::AuthUser, models::User};

static SERVER_START_TIME: OnceLock<Instant> = OnceLock::new();

pub fn init_server_start_time() {
    SERVER_START_TIME.get_or_init(Instant::now);
}

fn get_uptime_seconds() -> u64 {
    if let Some(start_time) = SERVER_START_TIME.get() {
        start_time.elapsed().as_secs()
    } else {
        0
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateAdminRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct CreateAdminResponse {
    pub success: bool,
    pub message: String,
    pub user_id: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub is_active: Option<bool>,
    pub is_admin: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct UserListResponse {
    pub users: Vec<User>,
    pub total: i64,
}

pub async fn create_admin_user(
    State(state): State<AppState>,
    Extension(admin_user): Extension<AuthUser>,
    Json(payload): Json<CreateAdminRequest>,
) -> Result<Json<CreateAdminResponse>, StatusCode> {
    info!("Admin {} attempting to create admin user: {}", admin_user.username, payload.username);

    match state.db.create_user(&payload.username, &payload.email, &payload.password, true).await {
        Ok(user_id) => {
            info!("Successfully created admin user {} with ID {}", payload.username, user_id);
            Ok(Json(CreateAdminResponse {
                success: true,
                message: "Admin user created successfully".to_string(),
                user_id: Some(user_id),
            }))
        }
        Err(e) => {
            error!("Failed to create admin user {}: {}", payload.username, e);
            if e.to_string().contains("unique constraint") {
                Ok(Json(CreateAdminResponse {
                    success: false,
                    message: "Username or email already exists".to_string(),
                    user_id: None,
                }))
            } else {
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}

pub async fn list_users(
    State(state): State<AppState>,
    Extension(admin_user): Extension<AuthUser>,
) -> Result<Json<UserListResponse>, StatusCode> {
    info!("Admin {} requesting user list", admin_user.username);

    match state.db.list_all_users().await {
        Ok(users) => {
            let total = users.len() as i64;
            Ok(Json(UserListResponse { users, total }))
        }
        Err(e) => {
            error!("Failed to list users: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_user(
    State(state): State<AppState>,
    Extension(admin_user): Extension<AuthUser>,
    Path(user_id): Path<i32>,
) -> Result<Json<User>, StatusCode> {
    info!("Admin {} requesting details for user ID: {}", admin_user.username, user_id);

    match state.db.get_user_by_id(user_id).await {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            error!("Failed to get user {}: {}", user_id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn update_user(
    State(state): State<AppState>,
    Extension(admin_user): Extension<AuthUser>,
    Path(user_id): Path<i32>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    info!("Admin {} updating user ID: {}", admin_user.username, user_id);

    if let Some(is_active) = payload.is_active {
        if let Err(e) = state.db.update_user_status(user_id, is_active).await {
            error!("Failed to update user status: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    if let Some(is_admin) = payload.is_admin {
        if let Err(e) = state.db.update_user_admin_status(user_id, is_admin).await {
            error!("Failed to update user admin status: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "User updated successfully"
    })))
}

pub async fn delete_user(
    State(state): State<AppState>,
    Extension(admin_user): Extension<AuthUser>,
    Path(user_id): Path<i32>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    info!("Admin {} deleting user ID: {}", admin_user.username, user_id);

    if admin_user.id == user_id {
        return Ok(Json(serde_json::json!({
            "success": false,
            "message": "Cannot delete your own account"
        })));
    }

    match state.db.delete_user(user_id).await {
        Ok(true) => Ok(Json(serde_json::json!({
            "success": true,
            "message": "User deleted successfully"
        }))),
        Ok(false) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            error!("Failed to delete user {}: {}", user_id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_user_chat_history(
    State(state): State<AppState>,
    Extension(admin_user): Extension<AuthUser>,
    Path(user_id): Path<i32>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    info!("Admin {} requesting chat history for user ID: {}", admin_user.username, user_id);

    match state.db.get_chat_history_by_user(user_id).await {
        Ok(history) => Ok(Json(serde_json::json!({
            "user_id": user_id,
            "chat_history": history
        }))),
        Err(e) => {
            error!("Failed to get chat history for user {}: {}", user_id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[derive(Debug, Serialize)]
pub struct OverviewStats {
    pub total_users: i64,
    pub active_users: i64,
    pub admin_users: i64,
    pub total_configs: usize,
    pub total_documents: usize,
    pub system_uptime: u64,
    pub service_status: ServiceStatus,
}

#[derive(Debug, Serialize)]
pub struct ServiceStatus {
    pub auth_api: String,
    pub bedrock_api: String,
    pub rag_api: String,
    pub database: String,
}

#[derive(Debug, Serialize)]
pub struct SystemHealth {
    pub status: String,
    pub uptime: u64,
    pub memory_usage: Option<u64>,
    pub disk_usage: Option<u64>,
    pub database_connection: bool,
    pub external_services: ServiceStatus,
}

#[derive(Debug, Serialize)]
pub struct SystemStats {
    pub users: UserStats,
    pub documents: DocumentStats,
    pub configurations: ConfigStats,
    pub recent_activity: Vec<ActivityLog>,
}

#[derive(Debug, Serialize)]
pub struct UserStats {
    pub total: i64,
    pub active: i64,
    pub admins: i64,
    pub recent_logins: i64,
}

#[derive(Debug, Serialize)]
pub struct DocumentStats {
    pub total_files: usize,
    pub total_folders: usize,
    pub total_size: u64,
}

#[derive(Debug, Serialize)]
pub struct ConfigStats {
    pub total: usize,
    pub encrypted: usize,
}

#[derive(Debug, Serialize)]
pub struct ActivityLog {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub action: String,
    pub user: String,
    pub details: String,
}

pub async fn admin_overview(
    State(state): State<AppState>,
    Extension(admin_user): Extension<AuthUser>,
) -> Result<Json<OverviewStats>, StatusCode> {
    info!("Admin {} requesting overview", admin_user.username);

    let total_users = state.db.get_user_count().await.unwrap_or(0);
    let active_users = state.db.get_active_user_count().await.unwrap_or(0);
    let admin_users = state.db.get_admin_user_count().await.unwrap_or(0);
    
    let configs = state.db.list_config_settings().await.unwrap_or_default();
    let total_configs = configs.len();

    let documents_count = state.db.get_document_count().await.unwrap_or(0);

    let uptime = get_uptime_seconds();

    let service_status = ServiceStatus {
        auth_api: check_service_health("http://localhost:9102/health").await,
        bedrock_api: check_service_health("http://localhost:9100/health").await,
        rag_api: check_service_health("http://localhost:9101/health").await,
        database: if state.db.test_connection().await { "healthy".to_string() } else { "unhealthy".to_string() },
    };

    Ok(Json(OverviewStats {
        total_users,
        active_users,
        admin_users,
        total_configs,
        total_documents: documents_count as usize,
        system_uptime: uptime,
        service_status,
    }))
}

pub async fn system_health(
    State(state): State<AppState>,
    Extension(admin_user): Extension<AuthUser>,
) -> Result<Json<SystemHealth>, StatusCode> {
    info!("Admin {} requesting system health", admin_user.username);

    let uptime = get_uptime_seconds();

    let database_connection = state.db.test_connection().await;

    let external_services = ServiceStatus {
        auth_api: check_service_health("http://localhost:9102/health").await,
        bedrock_api: check_service_health("http://localhost:9100/health").await,
        rag_api: check_service_health("http://localhost:9101/health").await,
        database: if database_connection { "healthy".to_string() } else { "unhealthy".to_string() },
    };

    let overall_status = if database_connection && 
        external_services.auth_api == "healthy" &&
        external_services.bedrock_api == "healthy" &&
        external_services.rag_api == "healthy" {
        "healthy"
    } else {
        "degraded"
    };

    Ok(Json(SystemHealth {
        status: overall_status.to_string(),
        uptime,
        memory_usage: get_memory_usage().await,
        disk_usage: get_disk_usage().await,
        database_connection,
        external_services,
    }))
}

pub async fn system_stats(
    State(state): State<AppState>,
    Extension(admin_user): Extension<AuthUser>,
) -> Result<Json<SystemStats>, StatusCode> {
    info!("Admin {} requesting system stats", admin_user.username);

    let total_users = state.db.get_user_count().await.unwrap_or(0);
    let active_users = state.db.get_active_user_count().await.unwrap_or(0);
    let admin_users = state.db.get_admin_user_count().await.unwrap_or(0);
    let recent_logins = state.db.get_recent_login_count().await.unwrap_or(0);

    let user_stats = UserStats {
        total: total_users,
        active: active_users,
        admins: admin_users,
        recent_logins,
    };

    let configs = state.db.list_config_settings().await.unwrap_or_default();
    let total_configs = configs.len();
    let encrypted_configs = configs.iter().filter(|c| c.is_encrypted.unwrap_or(false)).count();

    let config_stats = ConfigStats {
        total: total_configs,
        encrypted: encrypted_configs,
    };

    let (total_folders, total_files, total_size) = get_document_stats(&state.config.documents_base_path).await;
    let document_stats = DocumentStats {
        total_files,
        total_folders,
        total_size,
    };

    let recent_activity = state.db.get_recent_activity(10).await.unwrap_or_default();

    Ok(Json(SystemStats {
        users: user_stats,
        documents: document_stats,
        configurations: config_stats,
        recent_activity,
    }))
}

async fn check_service_health(url: &str) -> String {
    match reqwest::get(url).await {
        Ok(response) => {
            if response.status().is_success() {
                "healthy".to_string()
            } else {
                "unhealthy".to_string()
            }
        }
        Err(_) => "unreachable".to_string(),
    }
}

async fn get_memory_usage() -> Option<u64> {
    // This would require platform-specific code or a crate like `sysinfo`
    // For now, return None - can be implemented later
    None
}

async fn get_disk_usage() -> Option<u64> {
    // This would require platform-specific code or a crate like `sysinfo`
    // For now, return None - can be implemented later
    None
}

async fn get_document_stats(documents_path: &str) -> (usize, usize, u64) {
    use std::path::Path;
    use tokio::fs;

    let path = Path::new(documents_path);
    if !path.exists() {
        return (0, 0, 0);
    }

    let mut total_folders = 0;
    let mut total_files = 0;
    let mut total_size = 0u64;

    if let Ok(mut entries) = fs::read_dir(path).await {
        while let Ok(Some(entry)) = entries.next_entry().await {
            if let Ok(metadata) = entry.metadata().await {
                if metadata.is_dir() {
                    total_folders += 1;
                    // Count files in folder
                    if let Ok(mut folder_entries) = fs::read_dir(entry.path()).await {
                        while let Ok(Some(file_entry)) = folder_entries.next_entry().await {
                            if let Ok(file_metadata) = file_entry.metadata().await {
                                if file_metadata.is_file() {
                                    total_files += 1;
                                    total_size += file_metadata.len();
                                }
                            }
                        }
                    }
                } else if metadata.is_file() {
                    total_files += 1;
                    total_size += metadata.len();
                }
            }
        }
    }

    (total_folders, total_files, total_size)
}