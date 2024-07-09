use crate::exam::{Exam, ExamBuilder};
use crate::utils::{capture_number_input, generate_exam_questions};
use std::io;

pub struct BasicExamBuilder {
    pub(crate) exam: Exam,
}

impl ExamBuilder for BasicExamBuilder {
    fn choose_length(&mut self) -> &mut dyn ExamBuilder {
        let mut reader = io::stdin().lock();
        let length = capture_number_input(&mut reader);
        self.exam
            .questions
            .append(&mut generate_exam_questions(length));
        self
    }

    fn build(&mut self) -> Exam {
        self.exam.clone()
    }
}
