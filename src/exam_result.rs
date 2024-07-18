#[derive(Debug, Clone, Copy)]
pub struct ExamResult {
    pub score: i32,
    pub exam_length: i32,
    pub percentage: f32,
}

impl ExamResult {
    pub(crate) fn new(exam_length: i32) -> Self {
        Self {
            score: 0,
            exam_length,
            percentage: 0f32,
        }
    }

    pub fn increment(&mut self) {
        self.score += 1
    }

    pub fn calculate(&mut self) {
        self.percentage = (self.score as f32 / self.exam_length as f32) * 100f32;
    }

    pub fn print(&self) -> String {
        format!(
            "You scored {} out of {}: {}%",
            self.score, self.exam_length, self.percentage
        )
    }
}
