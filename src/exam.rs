use crate::exam_result::ExamResult;
use crate::question::Question;

#[derive(Debug, Clone)]
pub struct Exam {
    pub questions: Vec<Question>,
    pub current_question: Question,
    pub state: ExamState,
}

#[derive(Debug, Clone)]
pub enum ExamState {
    Initialised,
    Created,
    InProgress(Question),
    Finished(ExamResult),
}

pub trait ExamBuilder {
    fn build(&mut self) -> Exam;
}

impl Exam {
    pub fn new(state: ExamState) -> Self {
        Self {
            questions: Vec::new(),
            current_question: Question::placeholder(),
            state,
        }
    }
}
