use reqwest::{Client, header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE}};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct LangChainClient {
	client: Client,
	base_url: String,
}

#[derive(Serialize)]
struct GroqMessage {
	role: String,
	content: String,
}

#[derive(Serialize)]
struct GroqRequest {
	model: String,
	messages: Vec<GroqMessage>,
	max_tokens: u32,
	temperature: f32,
}

#[derive(Deserialize)]
struct GroqChoice {
	message: GroqMessageResponse,
}

#[derive(Deserialize)]
struct GroqMessageResponse {
	content: String,
}

#[derive(Deserialize)]
struct GroqResponse {
	choices: Vec<GroqChoice>,
}

#[derive(Debug)]
pub enum LangChainError {
	NetworkError(reqwest::Error),
	ApiError(String),
	ParseError(String),
}

impl From<reqwest::Error> for LangChainError {
	fn from(err: reqwest::Error) -> Self {
		LangChainError::NetworkError(err)
	}
}

impl LangChainClient {
	pub fn new(api_key: String) -> Result<Self, LangChainError> {
		let mut headers = HeaderMap::new();
		let auth_value = HeaderValue::from_str(&format!("Bearer {}", api_key))
			.map_err(|_| LangChainError::ApiError("Invalid API key format".to_string()))?;
		headers.insert(AUTHORIZATION, auth_value);
		headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

		let client = Client::builder()
			.default_headers(headers)
			.timeout(std::time::Duration::from_secs(30))
			.build()?;

		Ok(Self {
			client,
			base_url: "https://api.groq.com/openai/v1/chat/completions".to_string(),
		})
	}

	pub async fn ask_question(&self, subject: &str, question: &str) -> Result<String, LangChainError> {
		let prompt = self.create_subject_prompt(subject, question);

		let request_body = GroqRequest {
			model: "meta-llama/llama-4-scout-17b-16e-instruct".to_string(),
			messages: vec![
				GroqMessage { role: "user".to_string(), content: prompt },
			],
			max_tokens: 1000,
			temperature: 0.7,
		};

		println!("🤖 Sending request to Groq API for subject: {}", subject);
		println!("🔗 Endpoint: {}", self.base_url);

		let response = self.client.post(&self.base_url).json(&request_body).send().await?;
		let status = response.status();
		if !status.is_success() {
			let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
			println!("❌ API Error Response: {}", error_text);
			return Err(LangChainError::ApiError(format!(
				"Groq API request failed with status {}: {}", status, error_text
			)));
		}

		let groq_response: GroqResponse = response.json().await
			.map_err(|e| LangChainError::ParseError(format!("Failed to parse Groq response: {}", e)))?;

		if let Some(choice) = groq_response.choices.first() {
			Ok(choice.message.content.clone())
		} else {
			Err(LangChainError::ApiError("No response content from Groq".to_string()))
		}
	}

	fn create_subject_prompt(&self, subject: &str, question: &str) -> String {
		format!(
			"You are a helpful AI tutor. Provide a clear, educational answer with examples.\n\nSubject: {}\nQuestion: {}",
			subject, question
		)
	}
}
