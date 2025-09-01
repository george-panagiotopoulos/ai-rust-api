mod auth_client;
mod azure_client;
mod config;
mod error;
mod handlers;

use auth_client::AuthClient;
use azure_client::AzureClient;
use axum::{
    routing::{get, post},
    Router,
};
use config::Config;
use handlers::AppState;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing::{info, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "azure_api=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting AzureAPI server...");

    let config = Arc::new(Config::from_env()?);
    info!("Configuration loaded: {}", config.bind_address());

    let azure_client = Arc::new(AzureClient::new((*config).clone()));
    info!("Azure OpenAI client initialized with endpoint: {}", config.azure_endpoint);

    let auth_client = Arc::new(AuthClient::new(config.auth_api_url.clone()));
    info!("AuthClient initialized with URL: {}", config.auth_api_url);

    let state = AppState {
        azure_client,
        auth_client,
    };

    let app = Router::new()
        .route("/health", get(handlers::health))
        .route("/chat", post(handlers::chat))
        .route("/simple-chat", post(handlers::simple_chat))
        .route("/embeddings", post(handlers::create_embedding))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let bind_address = config.bind_address();
    let listener = tokio::net::TcpListener::bind(&bind_address).await?;
    info!("AzureAPI server running on http://{}", bind_address);

    axum::serve(listener, app).await?;

    Ok(())
}