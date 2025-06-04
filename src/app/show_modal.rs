use ratatui::{
    Frame,
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Paragraph},
};

use super::centered_rect;

pub fn show_modal(title: &str, frame: &mut Frame, buffer: &str) {
    let area = centered_rect(50, 3, frame.area());
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Yellow).bg(Color::Black));

    let input = Paragraph::new(buffer)
        .block(block)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left);

    frame.render_widget(Clear, area);
    frame.render_widget(input, area);
}
