use std::io;

use crate::question::{Question, QuestionType};
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

    pub fn with_length(mut self, length: i8) -> Self {
        self.questions.append(&mut generate_exam_questions(length));
        self
    }

    pub fn take_exam(&mut self) -> ExamResult {
        let mut score = 0;
        for question in &self.questions {
            println!("{:?}", question.text);
            if capture_guess(question).eq_ignore_ascii_case(&question.answer) {
                score += 1;
            }
        }
        ExamResult::new(score, self.questions.len() as i32)
    }
}

fn capture_guess(question: &Question) -> String {
    let mut guess = String::new();

    match &question.q_type {
        QuestionType::Capture => {
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");
            guess.trim().to_string()
        }
        QuestionType::MultipleChoice(data) => {
            println!("Possible options are: {:?}", data);
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");
            match data.iter().any(|s| s == guess.trim()) {
                true => guess.trim().to_string(),
                false => {
                    guess.clear();
                    println!("Enter a valid option");
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
