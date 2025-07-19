use ratatui::{Frame, layout::Rect, style::Stylize};

use crate::app::Error;

pub fn render_error(frame: &mut Frame, area: Rect, error: &Error) {
    frame.render_widget(format!(" {error} ").red(), area);
}
