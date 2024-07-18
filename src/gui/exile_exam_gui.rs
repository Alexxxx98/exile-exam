use iced::alignment::Horizontal::Center;
use iced::widget::{button, text, text_input, Column, Space, TextInput};
use iced::{Alignment, Element, Length, Sandbox, Theme};
use iced_aw::{Grid, GridRow};

use crate::basic_exam::BasicExamBuilder;
use crate::exam::{Exam, ExamBuilder, ExamState};
use crate::exam_result::ExamResult;
use crate::utils::generate_exam_questions;

pub struct ExileExamGUI {
    exam: Option<Exam>,
    exam_result: Option<ExamResult>,
    input_value: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    SelectBasicExam,
    EnterExamLength,
    EnterAnswer,
    ReturnMainMenu,
    StartExam,
}

impl Sandbox for ExileExamGUI {
    type Message = Message;

    fn new() -> Self {
        Self {
            exam: None,
            exam_result: None,
            input_value: "".to_string(),
        }
    }

    fn title(&self) -> String {
        "ExileExam".into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::InputChanged(input) => {
                self.input_value = input;
            }
            Message::SelectBasicExam => {
                self.exam = Some(
                    BasicExamBuilder {
                        exam: Exam::new(ExamState::Initialised),
                    }
                    .build(),
                );
            }
            Message::EnterExamLength => match self.input_value.parse::<usize>() {
                Ok(size) => {
                    let mut questions = generate_exam_questions(size);
                    self.input_value.clear();
                    if let Some(ref mut exam) = self.exam {
                        exam.questions.append(&mut questions);
                        exam.state = ExamState::Created
                    }
                }
                Err(_) => {
                    if let Some(ref mut exam) = self.exam {
                        exam.state = ExamState::Initialised
                    }
                }
            },
            Message::StartExam => {
                if let Some(ref mut exam) = self.exam {
                    self.exam_result = Some(ExamResult::new(exam.questions.len() as i32));
                    let question = exam.questions.pop().unwrap();
                    exam.current_question = question.clone();
                    exam.state = ExamState::InProgress(question);
                }
            }
            Message::EnterAnswer => {
                if let Some(ref mut exam) = self.exam {
                    if let Some(ref mut exam_result) = self.exam_result {
                        if exam
                            .current_question
                            .evaluate_answer(self.input_value.clone())
                        {
                            exam_result.increment();
                        }
                        self.input_value.clear();
                        match exam.questions.pop() {
                            None => exam.state = ExamState::Finished(exam_result.clone()),
                            Some(question) => {
                                exam.current_question = question.clone();
                                exam.state = ExamState::InProgress(question)
                            }
                        }
                    }
                }
            }
            Message::ReturnMainMenu => self.exam = None,
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let title = text("Exile Exam")
            .width(Length::Fill)
            .size(100)
            .horizontal_alignment(Center);

        let main_menu_button = button("Back to main menu").on_press(Message::ReturnMainMenu);

        if let Some(exam) = &self.exam {
            match &exam.state {
                ExamState::Initialised => {
                    let input = text_input("Enter length of exam", &self.input_value)
                        .on_input(Message::InputChanged)
                        .on_submit(Message::EnterExamLength)
                        .width(Length::Fill)
                        .size(100);
                    Column::new()
                        .push(title)
                        .push(input)
                        .align_items(Alignment::Center)
                        .into()
                }
                ExamState::Created => {
                    let begin_exam = text(format!(
                        "Exam created with {} questions",
                        exam.questions.len()
                    ))
                    .width(Length::Fill)
                    .size(60)
                    .horizontal_alignment(Center);
                    let button = button("Begin exam")
                        .on_press(Message::StartExam)
                        .padding(60);
                    Column::new()
                        .push(title)
                        .push(begin_exam)
                        .push(button)
                        .push(Space::with_height(Length::Fill))
                        .push(main_menu_button)
                        .align_items(Alignment::Center)
                        .into()
                }
                ExamState::InProgress(question) => {
                    let question = text(&question.text)
                        .width(Length::Fill)
                        .size(100)
                        .horizontal_alignment(Center);
                    let answer = TextInput::new("Enter length of exam", &self.input_value)
                        .on_input(Message::InputChanged)
                        .on_submit(Message::EnterAnswer)
                        .width(Length::Fill)
                        .size(100);

                    Column::new()
                        .push(title)
                        .push(question)
                        .push(answer)
                        .push(main_menu_button)
                        .align_items(Alignment::Center)
                        .into()
                }
                ExamState::Finished(mut result) => {
                    result.calculate();
                    let result = text(result.print().trim())
                        .width(Length::Fill)
                        .size(100)
                        .horizontal_alignment(Center);
                    Column::new()
                        .push(title)
                        .push(result)
                        .push(main_menu_button)
                        .align_items(Alignment::Center)
                        .into()
                }
            }
        } else {
            let button1 = button("Basic Exam")
                .on_press(Message::SelectBasicExam)
                .padding(60);
            let button2 = iced::widget::button("Basic Exam")
                .on_press(Message::SelectBasicExam)
                .padding(60);
            let button3 = iced::widget::button("Basic Exam")
                .on_press(Message::SelectBasicExam)
                .padding(60);
            let button4 = iced::widget::button("Basic Exam")
                .on_press(Message::SelectBasicExam)
                .padding(60);

            let grid = Grid::new()
                .column_spacing(60)
                .row_spacing(60)
                .push(GridRow::new().push(button1).push(button2))
                .push(GridRow::new().push(button3).push(button4));
            Column::new()
                .push(Space::with_height(40))
                .push(title)
                .push(Space::with_height(Length::Fill))
                .push(grid)
                .push(Space::with_height(Length::Fill))
                .align_items(Alignment::Center)
                .into()
        }
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
