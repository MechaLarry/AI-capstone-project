use regex::Regex;

pub struct Validator;

impl Validator {
    pub fn is_valid_subject(subject: &str) -> bool {
        let valid_subjects = [
            "mathematics", "science", "history", "literature",
            "physics", "chemistry", "biology", "computer_science",
            "psychology", "philosophy", "economics", "general"
        ];
        valid_subjects.contains(&subject)
    }

    pub fn is_reasonable_question_length(question: &str) -> bool {
        let len = question.trim().len();
        len >= 5 && len <= 1000
    }

    pub fn sanitize_input(input: &str) -> String {
        // Basic sanitization - remove potentially harmful characters
        let re = Regex::new(r#"[<>"'&]"#).unwrap();
        re.replace_all(input.trim(), "").to_string()
    }

    pub fn is_educational_question(question: &str) -> bool {
        // Basic check for educational content vs harmful requests
        let question_lower = question.to_lowercase();
        
        // Block obviously harmful requests
        let harmful_patterns = ["hack", "exploit", "illegal", "cheat on", "plagiarize"];
        if harmful_patterns.iter().any(|pattern| question_lower.contains(pattern)) {
            return false;
        }
        
        true
    }
}
