use crate::exam_result::ExamResult;
use std::io;
use std::io::BufRead;

use crate::question::{Question, QuestionType};

#[derive(Debug, Clone)]
pub struct Exam {
    pub questions: Vec<Question>,
}

pub trait ExamBuilder {
    fn choose_length(&mut self) -> &mut dyn ExamBuilder;
    fn build(&mut self) -> Exam;
}

impl Exam {
    pub fn new() -> Self {
        Self {
            questions: Vec::new(),
        }
    }

    pub fn take_exam(&mut self) -> ExamResult {
        let mut score = 0;
        let mut reader = io::stdin().lock();

        for question in &self.questions {
            println!("{:?}", question.text);
            if capture_guess(&mut reader, question).eq_ignore_ascii_case(&question.answer) {
                score += 1;
            }
        }
        ExamResult::new(score, self.questions.len() as i32)
    }
}

fn capture_guess<R: BufRead>(reader: &mut R, question: &Question) -> String {
    let mut guess = String::new();

    match &question.q_type {
        QuestionType::Capture => {
            reader.read_line(&mut guess).expect("Failed to read line");
            guess.trim().to_string()
        }
        QuestionType::MultipleChoice(data) => {
            println!("Possible options are: {:?}", data);
            reader.read_line(&mut guess).expect("Failed to read line");
            match data.iter().any(|s| s == guess.trim()) {
                true => guess.trim().to_string(),
                false => {
                    guess.clear();
                    println!("Enter a valid option");
                    capture_guess(reader, question)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_capture_guess() {
        let question = Question {
            id: 1,
            text: "Test".to_string(),
            q_type: QuestionType::Capture,
            answer: "test_answer".to_string(),
        };
        let mut reader = Cursor::new(b"test_answer\n");
        assert_eq!(
            String::from("test_answer"),
            capture_guess(&mut reader, &question)
        )
    }
}
