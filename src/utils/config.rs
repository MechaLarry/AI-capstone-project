use std::env;

pub struct Config {
    pub langchain_api_key: Option<String>,
    pub server_port: u16,
    pub max_tokens: u32,
    pub request_timeout: u64,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            langchain_api_key: env::var("LANGCHAIN_API_KEY").ok(),
            server_port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .unwrap_or(3000),
            max_tokens: env::var("MAX_TOKENS")
                .unwrap_or_else(|_| "500".to_string())
                .parse()
                .unwrap_or(500),
            request_timeout: env::var("REQUEST_TIMEOUT")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .unwrap_or(30),
        }
    }

    pub fn has_api_key(&self) -> bool {
        self.langchain_api_key.is_some()
    }

    pub fn get_api_key(&self) -> Option<&String> {
        self.langchain_api_key.as_ref()
    }
}
