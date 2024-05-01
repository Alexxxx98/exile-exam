use rand::Rng;

use crate::exam::Exam;

mod exam;
mod question;

fn main() {
    let mut exam = Exam::new().with_length(5);
    exam.take_exam().print();
}
