use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionRequest {
    pub subject: String,
    pub question: String,
}

impl QuestionRequest {
    pub fn new(subject: String, question: String) -> Self {
        Self { subject, question }
    }

    pub fn is_valid(&self) -> bool {
        !self.subject.trim().is_empty() && !self.question.trim().is_empty()
    }
}
