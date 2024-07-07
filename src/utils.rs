use crate::question::Question;
use rand::seq::IteratorRandom;

pub fn generate_exam_questions(length: i8) -> Vec<Question> {
    let json_file =
        std::fs::read_to_string("src/resources/example-questions.json").expect("Cannot read file");
    let questions: Vec<Question> = serde_json::from_str(&json_file).expect("Cannot serialise");
    questions
        .iter()
        .choose_multiple(&mut rand::thread_rng(), length as usize)
        .into_iter()
        .cloned()
        .collect()
}
