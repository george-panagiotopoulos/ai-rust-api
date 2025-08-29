mod auth_client;
mod bedrock_client;
mod config;
mod database;
mod embeddings;
mod handlers;
mod rag;

use anyhow::Result;
use auth_client::AuthClient;
use aws_config::BehaviorVersion;
use aws_sdk_bedrockruntime::Client as BedrockClient;
use axum::{
    routing::{get, post},
    Router,
};
use config::Config;
use database::Database;
use embeddings::EmbeddingService;
use handlers::{health, query, search_documents, stats, process_document, generate_embedding, process_stored_document, get_rag_models, AppState};
use rag::RAGService;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tracing::{info, Level};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("Starting RAGAPI server...");

    // Load configuration
    let config = Config::from_env()?;
    info!("Configuration loaded: {}:{}", config.host, config.port);

    // Initialize database connection
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await?;

    info!("Connected to database");

    // Run database migrations if needed  
    // sqlx::migrate!("./migrations").run(&pool).await?;
    info!("Database connection ready");

    // Initialize AWS Bedrock client
    let aws_config = aws_config::defaults(BehaviorVersion::latest())
        .region(aws_config::Region::new(config.aws_region.clone()))
        .load()
        .await;
    
    let bedrock_client = BedrockClient::new(&aws_config);
    info!("AWS Bedrock client initialized");

    // Initialize services
    let database = Database::new(pool);
    let embedding_service = EmbeddingService::new(bedrock_client);

    // Skip automatic embedding generation during startup due to AWS credential issues
    // Will generate embeddings manually when needed
    info!("Skipping automatic embedding generation during startup");

    let bedrock_api_client = bedrock_client::BedrockApiClient::new(config.bedrock_api_url.clone());
    let rag_service = RAGService::new(database, embedding_service, bedrock_api_client);
    
    // Initialize AuthClient
    let auth_client = AuthClient::new(config.auth_api_url.clone());
    info!("AuthClient initialized with URL: {}", config.auth_api_url);

    // Create shared state
    let app_state = AppState {
        rag_service: Arc::new(rag_service),
        auth_client: Arc::new(auth_client),
    };

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build the router
    let app = Router::new()
        .route("/health", get(health))
        .route("/stats", get(stats))
        .route("/query", post(query))
        .route("/search", post(search_documents))
        .route("/process-document", post(process_document))
        .route("/generate-embedding", post(generate_embedding))
        .route("/process-stored-document", post(process_stored_document))
        .route("/rag-models", get(get_rag_models))
        .with_state(app_state)
        .layer(ServiceBuilder::new().layer(cors));

    // Start the server
    let bind_address = config.bind_address();
    info!("Server starting on {}", bind_address);

    let listener = tokio::net::TcpListener::bind(&bind_address).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
