#![feature(exclusive_range_pattern)]

use rand::Rng;

use crate::exam::Exam;

mod exam;

fn main() {
    let quiz_length = 5u8;
    let mut exam = Exam::new().with_length(quiz_length);
    println!("{}", exam.take_exam());
}
