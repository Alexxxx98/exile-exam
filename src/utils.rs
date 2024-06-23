use crate::question::Question;

pub fn generate_exam_questions(length: u8) -> Vec<Question> {
    let json_file =
        std::fs::read_to_string("src/resources/example-questions.json").expect("Cannot read file");
    let questions: Vec<Question> = serde_json::from_str(&json_file).expect("Cannot serialise");
    println!("{:?}", questions);
    questions
}
