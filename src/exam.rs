use std::any::Any;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

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

    pub fn take_exam(&mut self) -> String {
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
        format!("You scored {} out of {}", score, self.questions.len())
    }
}

fn capture_guess(question: &Question) -> i32 {
    match question.q_type {
        Type::MultipleChoice(data) => {
            println!("Possible options are: {:?}", data);

            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            guess.trim().parse().expect("Please enter a number")
        }
        Type::Capture => {
            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            guess.trim().parse().expect("Please enter a number")
        }
    }
}

#[derive(Debug)]
pub struct Question {
    id: u8,
    text: String,
    q_type: Type,
    answer: i32,
}

#[derive(Debug)]
enum Type {
    MultipleChoice([i32; 4]),
    Capture,
}

impl Question {
    fn new(id: u8, text: String, q_type: Type, answer: i32) -> Self {
        Self {
            id,
            text,
            q_type,
            answer,
        }
    }
}
