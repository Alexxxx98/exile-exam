use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    id: u8,
    pub(crate) text: String,
    pub(crate) q_type: Type,
    pub(crate) answer: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    MultipleChoice([i32; 4]),
    Capture,
}

impl Question {
    pub(crate) fn new(id: u8, text: String, q_type: Type, answer: i32) -> Self {
        Self {
            id,
            text,
            q_type,
            answer,
        }
    }
}
