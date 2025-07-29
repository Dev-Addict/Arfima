use ratatui::{Frame, layout::Rect, text::Line};

use crate::app::precommand::Precommand;

pub fn render_precommand(frame: &mut Frame, area: Rect, precommand: &Precommand) {
    frame.render_widget(
        Line::from(match precommand {
            Precommand::Leader => " <leader> ".to_string(),
            Precommand::Repeat(repeat) => format!(" {repeat} "),
            Precommand::RepeatWindow(repeat) => format!(" {repeat}^W "),
        }),
        area,
    );
}
