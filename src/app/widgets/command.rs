use ratatui::{Frame, layout::Rect, text::Text};

use crate::app::widgets::{get_input, types::InputState};

pub fn render_command(frame: &mut Frame, area: Rect, state: &InputState) {
    let input = get_input(state);

    frame.render_widget(
        Text::from(" :"),
        Rect {
            x: area.x,
            y: area.y,
            width: 2,
            height: area.height,
        },
    );
    frame.render_widget(
        input,
        Rect {
            x: area.x + 2,
            y: area.y,
            width: area.width - 2,
            height: area.height,
        },
    );
}
