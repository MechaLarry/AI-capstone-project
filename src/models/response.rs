use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnswerResponse {
    pub answer: String,
    pub subject: String,
    pub status: String,
}

impl AnswerResponse {
    pub fn new(answer: String, subject: String) -> Self {
        Self {
            answer,
            subject,
            status: "success".to_string(),
        }
    }
}
