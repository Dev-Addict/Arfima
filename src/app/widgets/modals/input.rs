use ratatui::{
    Frame,
    layout::Alignment,
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};

use crate::{
    app::widgets::{centered_rect::get_centered_rect, types::InputState},
    utils::{all_but_first, first_char_str},
};

pub fn show_input_modal(title: &str, frame: &mut Frame, state: &InputState) {
    let area = get_centered_rect(50, 3, frame.area());
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Yellow).bg(Color::Black));

    let mut spans = Vec::new();

    let cursor_position = state.cursor_position().min(state.buffer().len());
    let (left, right) = state.buffer().split_at(cursor_position);

    spans.push(Span::raw(left));
    if cursor_position == state.buffer().len() {
        spans.push(Span::styled(
            "█",
            Style::default().fg(Color::White).bg(Color::Black).bold(),
        ));
        spans.push(Span::raw(right));
    } else {
        spans.push(Span::styled(
            first_char_str(right).unwrap_or("█"),
            Style::default().fg(Color::DarkGray).bg(Color::White).bold(),
        ));
        spans.push(Span::raw(all_but_first(right)));
    }

    let input = Paragraph::new(Line::from(spans))
        .block(block)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left);

    frame.render_widget(Clear, area);
    frame.render_widget(input, area);
}
