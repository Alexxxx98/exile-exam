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
