use crate::exam::Exam;

mod exam;
mod question;
mod utils;

fn main() {
    let mut exam = Exam::new().with_length(5);
    exam.take_exam().print();
}
