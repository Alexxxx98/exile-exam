use iced::{Application, Font, Settings};

use gui::exile_exam_gui::ExileExamGUI;

mod basic_exam;
mod exam;
mod exam_result;
mod gui;
mod question;
mod utils;

fn main() -> iced::Result {
    ExileExamGUI::run(Settings {
        fonts: vec![
            include_bytes!("./resources/fonts/Fontin-Regular.ttf")
                .as_slice()
                .into(),
            include_bytes!("./resources/fonts/Fontin-Bold.ttf")
                .as_slice()
                .into(),
            include_bytes!("./resources/fonts/Fontin-Italic.ttf")
                .as_slice()
                .into(),
            include_bytes!("./resources/fonts/Fontin-SmallCaps.ttf")
                .as_slice()
                .into(),
        ],
        default_font: Font::with_name("Fontin"),
        ..Settings::default()
    })
}
