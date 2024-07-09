use crate::basic_exam::BasicExamBuilder;
use crate::exam::{Exam, ExamBuilder};

mod basic_exam;
mod exam;
mod exam_result;
mod question;
mod utils;

fn main() {
    BasicExamBuilder { exam: Exam::new() }
        .choose_length()
        .build()
        .take_exam()
        .print();
}
