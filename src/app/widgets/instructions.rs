use ratatui::{Frame, layout::Rect, style::Stylize, text::Line};

pub fn render_instructions(frame: &mut Frame, area: Rect) {
    frame.render_widget(
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
        ]),
        area,
    );
}
