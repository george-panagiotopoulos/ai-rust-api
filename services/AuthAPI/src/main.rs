use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing::{info, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod auth;
mod config;
mod database;
mod error;
mod handlers;
mod models;

use auth::JwtManager;
use config::Config;
use database::Database;
use handlers::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "auth_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting Authentication API server...");

    // Load configuration
    let config = Config::from_env()?;
    info!("Configuration loaded: {}:{}", config.host, config.port);

    // Initialize database
    let database = Database::new(&config.database_url).await
        .map_err(|e| {
            error!("Failed to connect to database: {}", e);
            anyhow::anyhow!("Database connection failed: {}", e)
        })?;
    info!("Connected to database");

    // Initialize JWT manager
    let jwt_manager = JwtManager::new(config.jwt_secret.clone(), config.jwt_expiry_hours);

    // Create app state
    let app_state = Arc::new(AppState {
        db: database,
        jwt_manager,
        bcrypt_cost: config.bcrypt_cost,
    });

    // Create router
    let app = Router::new()
        .route("/health", get(handlers::health))
        .route("/register", post(handlers::register))
        .route("/login", post(handlers::login))
        .route("/logout", post(handlers::logout))
        .route("/validate", post(handlers::validate_token))
        .route("/profile", get(handlers::get_profile))
        .route("/stats", get(handlers::stats))
        .layer(CorsLayer::permissive())
        .with_state(app_state);

    // Start server
    let bind_address = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&bind_address).await?;
    info!("Server running on http://{}", bind_address);

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await?;

    Ok(())
}