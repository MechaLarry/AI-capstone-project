// Run with: cargo run --example test_client
use reqwest::Client;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    
    println!("🧪 Testing AI Tutor API...");
    
    // Test health endpoint
    let health_response = client
        .get("http://localhost:3000/health")
        .send()
        .await?;
    
    println!("Health check: {}", health_response.status());
    
    // Test question endpoint
    let question_data = json!({
        "subject": "mathematics",
        "question": "What is 2 + 2?"
    });
    
    let response = client
        .post("http://localhost:3000/api/ask")
        .json(&question_data)
        .send()
        .await?;
    
    println!("Question response status: {}", response.status());
    
    if response.status().is_success() {
        let body: serde_json::Value = response.json().await?;
        println!("Response: {:#}", body);
    } else {
        let error_text = response.text().await?;
        println!("Error: {}", error_text);
    }
    
    Ok(())
}
