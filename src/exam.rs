use std::cmp::Ordering;
use std::io;

use crate::question::{Question, Type};
use crate::utils::generate_exam_questions;

#[derive(Debug)]
pub struct Exam {
    pub questions: Vec<Question>,
}

impl Exam {
    pub fn new() -> Self {
        Self {
            questions: Vec::new(),
        }
    }

    pub fn with_length(mut self, length: u8) -> Self {
        for q in generate_exam_questions(length) {
            self.questions.push(q)
        }
        self
    }

    pub fn take_exam(&mut self) -> ExamResult {
        let mut score = 0;
        for question in &self.questions {
            println!("{:?}", question.text);

            match capture_guess(question).cmp(&question.answer) {
                Ordering::Equal => {
                    score += 1;
                }
                _ => continue,
            }
        }
        ExamResult::new(score, self.questions.len() as i32)
    }
}

fn capture_guess(question: &Question) -> i32 {
    let mut guess = String::new();

    match question.q_type {
        Type::MultipleChoice(data) => {
            println!("Possible options are: {:?}", data);
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");
            match guess.trim().parse::<i32>() {
                Ok(num) => {
                    if !data.contains(&num) {
                        guess.clear();
                        println!("Please enter one of the valid options");
                        capture_guess(question)
                    } else {
                        num
                    }
                }
                Err(_) => {
                    guess.clear();
                    println!("Please enter a number");
                    capture_guess(question)
                }
            }
        }
        Type::Capture => {
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");
            match guess.trim().parse::<i32>() {
                Ok(num) => num,
                Err(_) => {
                    guess.clear();
                    println!("Please enter a number");
                    capture_guess(question)
                }
            }
        }
    }
}

pub struct ExamResult {
    pub score: i32,
    pub exam_length: i32,
    pub percentage: f32,
}

impl ExamResult {
    fn new(score: i32, exam_length: i32) -> Self {
        Self {
            score,
            exam_length,
            percentage: (score as f32 / exam_length as f32) * 100f32,
        }
    }

    pub fn print(&self) {
        print!(
            "{}",
            format!(
                "You scored {} out of {}: {}%",
                self.score, self.exam_length, self.percentage
            )
        );
    }
}
