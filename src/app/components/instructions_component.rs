use ratatui::{style::Stylize, text::Line};

pub struct InstructionsComponent;

impl InstructionsComponent {
    pub fn get() -> Line<'static> {
        Line::from(vec![
            " Up ".into(),
            "<K>".blue().bold(),
            " Down ".into(),
            "<J>".blue().bold(),
            " Back ".into(),
            "<H>".blue().bold(),
            " Down ".into(),
            "<J>".blue().bold(),
            " Help ".into(),
            "<Ctrl+H>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ])
    }
}
