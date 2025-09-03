use axum::{extract::Json, http::StatusCode, response::Json as ResponseJson};
use crate::models::{QuestionRequest, AnswerResponse};
use crate::services::langchain_service::LangChainClient;

pub async fn handle_question(
    Json(request): Json<QuestionRequest>,
) -> Result<ResponseJson<AnswerResponse>, StatusCode> {
    println!("📚 Processing question about {}: {}", request.subject, request.question);
    
    // Get Groq API key
    let api_key = std::env::var("GROQ_API_KEY")
        .map_err(|e| {
            eprintln!("❌ Groq API key not found. Please set GROQ_API_KEY in your .env file");
            eprintln!("❌ Error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    println!("🔑 Groq API key found: {}...{}", &api_key[..8], &api_key[api_key.len()-4..]);
    
    let client = LangChainClient::new(api_key)
        .map_err(|e| {
            eprintln!("❌ Failed to create Groq API client: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    println!("🤖 Groq API client created successfully");
    
    let answer = client
        .ask_question(&request.subject, &request.question)
        .await
        .map_err(|e| {
            eprintln!("❌ Groq API error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    println!("✅ Got answer from Groq: {}...", &answer[..answer.len().min(100)]);
    
    let response = AnswerResponse {
        answer,
        subject: request.subject,
        status: "success".to_string(),
    };
    
    Ok(ResponseJson(response))
}
