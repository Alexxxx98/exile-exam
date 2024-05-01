use std::cmp::Ordering;
use std::io;

use rand::Rng;
use crate::question::{Question, Type};

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
        for n in 1..=length {
            let q_type = match rand::thread_rng().gen_range(0..=1) {
                0 => Type::Capture,
                _ => Type::MultipleChoice([1, 2, 3, 4])
            };
            let answer = match q_type {
                Type::MultipleChoice(data) => {
                    let i = data.get(rand::thread_rng().gen_range(0..data.len()));
                    match i {
                        None => {
                            panic!("Unable to select answer for multiple choice question")
                        }
                        Some(data) => {
                            data.to_owned()
                        }
                    }
                }
                Type::Capture => {
                    rand::thread_rng().gen_range(0..1)
                }
            };
            self.questions.push(Question::new(
                n,
                "What is the secret number".to_string(),
                q_type,
                answer,
            ))
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
                _ => continue
            }
        }
        ExamResult::new(score, self.questions.len() as i32)
    }
}

fn capture_guess(question: &Question) -> i32 {
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    match question.q_type {
        Type::MultipleChoice(data) => {
            println!("Possible options are: {:?}", data);
                match guess.trim().parse::<i32>() {
                    Ok(num) => {
                        if (!data.contains(&num)) {
                            guess.clear();
                            println!("Please enter one of the valid options");
                            capture_guess(question)
                        } else {
                            num
                        }
                    },
                    Err(_) => {
                        guess.clear();
                        println!("Please enter a number");
                        capture_guess(question)
                    }
                }
        }
        Type::Capture => {
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
            percentage: (score as f32 / exam_length as f32) * 100f32
        }
    }

    pub fn print(&self) {
        print!("{}", format!("You scored {} out of {}: {}%", self.score, self.exam_length, self.percentage));
    }
}