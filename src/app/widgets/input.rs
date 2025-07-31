use ratatui::{
    layout::Alignment,
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::{
    app::widgets::types::InputState,
    utils::str::{all_but_first, first_char_str},
};

pub fn get_input(state: &InputState) -> Paragraph {
    let mut spans = Vec::new();

    let cursor_position = state.cursor_position().min(state.buffer().len());
    let (left, right) = state.buffer_split();

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

    Paragraph::new(Line::from(spans))
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left)
}
