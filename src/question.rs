use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub(crate) id: u8,
    pub(crate) text: String,
    pub(crate) q_type: QuestionType,
    pub(crate) answer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuestionType {
    MultipleChoice([String; 4]),
    Capture,
}

impl Question {
    pub fn placeholder() -> Self {
        Self {
            id: 1,
            text: "".to_string(),
            q_type: QuestionType::Capture,
            answer: "".to_string(),
        }
    }
    pub fn evaluate_answer(&mut self, user_input: String) -> bool {
        self.answer.eq_ignore_ascii_case(user_input.trim())
    }
}
