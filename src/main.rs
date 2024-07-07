use crate::exam::Exam;
use std::io;

mod exam;
mod question;
mod utils;

fn main() {
    let mut exam = Exam::new().with_length(capture_length(String::new()));

    exam.take_exam().print();
}

fn capture_length(mut input: String) -> i8 {
    println!("Enter length of your exam");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    match input.trim().parse::<i8>() {
        Ok(num) => num,
        Err(_) => {
            input.clear();
            println!("Please enter a number");
            capture_length(input)
        }
    }
}
