pub struct ExamResult {
    pub score: i32,
    pub exam_length: i32,
    pub percentage: f32,
}

impl ExamResult {
    pub(crate) fn new(score: i32, exam_length: i32) -> Self {
        Self {
            score,
            exam_length,
            percentage: (score as f32 / exam_length as f32) * 100f32,
        }
    }

    pub fn print(&self) {
        println!(
            "{}",
            format!(
                "You scored {} out of {}: {}%",
                self.score, self.exam_length, self.percentage
            )
        );
    }
}
