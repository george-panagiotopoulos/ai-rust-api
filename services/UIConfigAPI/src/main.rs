mod config;
mod database;
mod models;
mod middleware;
mod handlers;
mod utils;
mod env_manager;

use axum::{
    extract::DefaultBodyLimit,
    middleware as axum_middleware,
    routing::{get, post, put, delete},
    Router,
    Json,
};
use config::Config;
use database::Database;
use middleware::AuthClient;
use env_manager::EnvManager;
use serde_json::json;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing::{info, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Database>,
    pub config: Arc<Config>,
    pub auth_client: Arc<AuthClient>,
    pub env_manager: Arc<EnvManager>,
}

async fn health() -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "service": "UIConfigAPI",
        "version": "0.1.0"
    }))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "ui_config_api=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting UIConfigAPI server...");

    // Initialize server start time for uptime tracking
    handlers::admin::init_server_start_time();

    let config = Arc::new(Config::from_env()?);
    info!("Configuration loaded: {}", config.bind_address());

    let db = Arc::new(Database::new(&config.database_url).await?);
    info!("Connected to database");

    let auth_client = Arc::new(AuthClient::new(config.auth_api_url.clone()));
    info!("AuthClient initialized with URL: {}", config.auth_api_url);

    // Initialize EnvManager with base path (go up from UIConfigAPI to ai-rust-api root)
    let base_path = std::env::current_dir()?.parent().unwrap().parent().unwrap().to_path_buf();
    let env_manager = Arc::new(EnvManager::new(base_path.to_string_lossy().to_string()));
    info!("EnvManager initialized with base path: {}", base_path.display());

    let state = AppState {
        db,
        config,
        auth_client,
        env_manager,
    };

    // Auth middleware is applied inline in the router configuration

    let public_routes = Router::new()
        .route("/health", get(health))
        .route("/auth/register", post(handlers::register))
        .route("/auth/chat", post(handlers::chat));

    let user_routes = Router::new()
        .route("/rag-models", get(handlers::list_rag_models_public))
        .layer(axum_middleware::from_fn_with_state(state.auth_client.clone(), middleware::require_user));

    let admin_routes = Router::new()
        // Admin dashboard routes
        .route("/admin/overview", get(handlers::admin_overview))
        .route("/admin/system/health", get(handlers::system_health))
        .route("/admin/system/stats", get(handlers::system_stats))
        // User management
        .route("/admin/users", get(handlers::list_users))
        .route("/admin/users", post(handlers::create_admin_user))
        .route("/admin/users/:id", get(handlers::get_user))
        .route("/admin/users/:id", put(handlers::update_user))
        .route("/admin/users/:id", delete(handlers::delete_user))
        .route("/admin/users/:id/chat-history", get(handlers::get_user_chat_history))
        // Configuration management (legacy database configs)
        .route("/admin/configs", get(handlers::list_configs))
        .route("/admin/configs", post(handlers::create_config))
        .route("/admin/configs/backup", get(handlers::backup_configs))
        .route("/admin/configs/:key", get(handlers::get_config))
        .route("/admin/configs/:key", put(handlers::update_config))
        .route("/admin/configs/:key", delete(handlers::delete_config))
        // New .env configuration management
        .route("/admin/env-configs", get(handlers::list_env_configs))
        .route("/admin/env-configs/validate", get(handlers::validate_env_configs))
        .route("/admin/env-configs/update", put(handlers::update_env_config))
        .route("/admin/env-configs/:key", get(handlers::get_env_config))
        // Document management
        .route("/admin/documents/folders", get(handlers::list_folders))
        .route("/admin/documents/folders", post(handlers::create_folder))
        .route("/admin/documents/folders/:folder/documents", get(handlers::list_documents))
        .route("/admin/documents/folders/:folder/upload", post(handlers::upload_document))
        .route("/admin/documents/folders/:folder/documents/:filename", delete(handlers::delete_document))
        .route("/admin/documents/folders/:folder", delete(handlers::delete_folder))
        // Vector management
        .route("/admin/vectors", get(handlers::list_vectors))
        .route("/admin/vectors", post(handlers::create_vector))
        .route("/admin/vectors/:id", get(handlers::get_vector))
        .route("/admin/vectors/:id", delete(handlers::delete_vector))
        // RAG Model management
        .route("/admin/rag-models", get(handlers::list_rag_models))
        .route("/admin/rag-models", post(handlers::create_rag_model))
        .route("/admin/rag-models/:id", get(handlers::get_rag_model))
        .route("/admin/rag-models/:id", put(handlers::update_rag_model))
        .route("/admin/rag-models/:id", delete(handlers::delete_rag_model))
        // Backend configuration management
        .route("/admin/backends", get(handlers::get_backend_configs))
        .route("/admin/backends/:provider", put(handlers::update_backend_config))
        .route("/admin/backends/:provider/activate", post(handlers::set_active_backend))
        .route("/admin/backends/status", get(handlers::get_backend_status))
        .route("/admin/backends/:provider/test", post(handlers::test_backend_connection))
        .layer(axum_middleware::from_fn_with_state(state.auth_client.clone(), middleware::require_admin));

    let app = public_routes
        .merge(user_routes)
        .merge(admin_routes)
        .layer(DefaultBodyLimit::max(20 * 1024 * 1024)) // 20MB limit
        .layer(CorsLayer::permissive())
        .with_state(state.clone());

    let bind_address = state.config.bind_address();
    let listener = tokio::net::TcpListener::bind(&bind_address).await?;
    info!("Server running on http://{}", bind_address);

    axum::serve(listener, app).await?;

    Ok(())
}