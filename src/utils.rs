use crate::question::Question;
use rand::seq::IteratorRandom;
use std::io::BufRead;

pub fn generate_exam_questions(length: usize) -> Vec<Question> {
    let json_file =
        std::fs::read_to_string("src/resources/example-questions.json").expect("Cannot read file");
    let questions: Vec<Question> = serde_json::from_str(&json_file).expect("Cannot serialise");
    questions
        .iter()
        .choose_multiple(&mut rand::thread_rng(), length)
        .into_iter()
        .cloned()
        .collect()
}

pub fn capture_number_input<R: BufRead>(reader: &mut R) -> usize {
    let mut input = String::new();
    println!("Enter length of your exam");
    reader.read_line(&mut input).expect("Failed to read line");
    match input.trim().parse::<usize>() {
        Ok(num) => num,
        Err(_) => {
            input.clear();
            println!("Please enter a number");
            capture_number_input(reader)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_generate_exam_questions_with_size_1() {
        let questions = generate_exam_questions(1);
        assert_eq!(1, questions.len())
    }

    #[test]
    fn test_capture_number_input_with_valid() {
        let mut reader = Cursor::new(b"3\n");
        assert_eq!(3, capture_number_input(&mut reader))
    }
}
