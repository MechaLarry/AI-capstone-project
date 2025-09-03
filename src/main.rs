

use axum::{
    routing::{get, post},
    Router,
    response::Json,
};
use tower_http::{
    services::ServeDir,
    cors::{CorsLayer, Any},
};
use serde_json::json;
use std::net::SocketAddr;
use dotenv::dotenv;

mod models;
mod services;
mod handlers;
mod utils;

use handlers::handle_question;

#[tokio::main]
async fn main() {
    println!("🔧 Loading environment variables...");
    match dotenv() {
        Ok(_) => println!("✅ .env file loaded successfully"),
        Err(e) => println!("⚠️  .env file not found or error loading: {:?}", e),
    }
    
    // Check if Groq API key is available
    match std::env::var("GROQ_API_KEY") {
        Ok(key) => println!("🔑 Groq API key found: {}...{}", &key[..8], &key[key.len()-4..]),
        Err(e) => println!("❌ Groq API key not found: {:?}", e),
    }
    
    println!("🚀 Starting AI Tutor server...");
    
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/ask", post(handle_question))
        .nest_service("/", ServeDir::new("static"))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🌐 Server listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");
        
    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}

async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "service": "AI Tutor",
        "version": "1.0.0"
    }))
}
