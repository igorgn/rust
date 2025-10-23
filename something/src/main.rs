mod models;
mod storage;
mod handlers;
mod error;

use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use storage::UrlStore;
use tower::ServiceBuilder;

/// The main application state shared across all handlers
/// 
/// This demonstrates the use of Arc (Atomic Reference Counting) for
/// thread-safe shared state in an async context.
#[derive(Clone)]
pub struct AppState {
    store: Arc<UrlStore>,
}

impl AppState {
    fn new() -> Self {
        Self {
            store: Arc::new(UrlStore::new()),
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the application state
    let state = AppState::new();

    // Build the router with all routes
    // Notice the layered architecture: routes -> handlers -> storage
    let app = Router::new()
        // API routes for creating and managing URLs
        .route("/api/shorten", post(handlers::create_short_url))
        .route("/api/urls", get(handlers::list_urls))
        .route("/api/stats/:short_code", get(handlers::get_stats))
        
        // Redirect route - the main functionality
        .route("/:short_code", get(handlers::redirect_to_url))
        
        // Health check endpoint
        .route("/health", get(handlers::health_check))
        
        // Share state across all handlers
        .with_state(state)
        
        // Add middleware for logging and error handling
        .layer(ServiceBuilder::new());

    // Bind to localhost:3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await?;
    
    println!("🚀 URL Shortener running on http://127.0.0.1:3000");
    println!("📝 API Endpoints:");
    println!("   POST /api/shorten - Create a short URL");
    println!("   GET  /api/urls    - List all URLs");
    println!("   GET  /api/stats/:short_code - Get URL statistics");
    println!("   GET  /:short_code - Redirect to original URL");
    println!("   GET  /health      - Health check");
    
    // Start the server
    axum::serve(listener, app).await?;

    Ok(())
}