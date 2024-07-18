use crate::question::Question;
use rand::seq::IteratorRandom;

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_generate_exam_questions_with_size_1() {
        let questions = generate_exam_questions(1);
        assert_eq!(1, questions.len())
    }
}
