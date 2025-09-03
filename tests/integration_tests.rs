use ai_tutor::models::{QuestionRequest, AnswerResponse};
use ai_tutor::services::PromptService;

#[tokio::test]
async fn test_question_request_validation() {
    let valid_request = QuestionRequest::new(
        "mathematics".to_string(),
        "What is calculus?".to_string()
    );
    assert!(valid_request.is_valid());

    let invalid_request = QuestionRequest::new(
        "".to_string(),
        "".to_string()
    );
    assert!(!invalid_request.is_valid());
}

#[test]
fn test_prompt_generation() {
    let prompt = PromptService::get_subject_prompt("mathematics", "What is algebra?");
    assert!(prompt.contains("mathematics tutor"));
    assert!(prompt.contains("What is algebra?"));
}

#[test]
fn test_available_subjects() {
    let subjects = PromptService::get_available_subjects();
    assert!(!subjects.is_empty());
    assert!(subjects.iter().any(|(key, _)| *key == "mathematics"));
}

#[test]
fn test_response_creation() {
    let response = AnswerResponse::new(
        "Test answer".to_string(),
        "mathematics".to_string()
    );
    assert_eq!(response.status, "success");
    assert_eq!(response.subject, "mathematics");
}
