use crate::exam::{Exam, ExamBuilder};

pub struct BasicExamBuilder {
    pub(crate) exam: Exam,
}

impl ExamBuilder for BasicExamBuilder {
    fn build(&mut self) -> Exam {
        self.exam.clone()
    }
}
