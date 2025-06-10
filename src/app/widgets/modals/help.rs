use ratatui::{
    Frame,
    style::{Color, Style},
    widgets::{Block, Borders, Clear, TableState},
};

use crate::app::widgets::{centered_rect::get_centered_rect_by_percent, get_help_table};

pub fn show_help_modal(frame: &mut Frame, selected_index: usize) {
    let area = get_centered_rect_by_percent(80, 70, frame.area());

    let block = Block::default()
        .title("Help")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Yellow).bg(Color::Black));

    let table = get_help_table(area.width).block(block);

    let mut state = TableState::default();
    state.select(Some(selected_index));

    frame.render_widget(Clear, area);
    frame.render_stateful_widget(table, area, &mut state);
}
