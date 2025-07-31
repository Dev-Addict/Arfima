use ratatui::{
    Frame,
    style::{Color, Style},
    widgets::{Block, Borders, Clear},
};

use crate::app::widgets::{centered_rect::get_centered_rect, get_input, types::InputState};

pub fn show_input_modal(title: &str, frame: &mut Frame, state: &InputState) {
    let area = get_centered_rect(50, 3, frame.area());
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Yellow).bg(Color::Black));

    let input = get_input(state).block(block);

    frame.render_widget(Clear, area);
    frame.render_widget(input, area);
}
