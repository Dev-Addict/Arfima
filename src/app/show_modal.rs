use ratatui::{
    Frame,
    layout::Alignment,
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};

use crate::utils::all_but_first;

use super::{InputState, centered_rect};

pub fn show_input_modal(title: &str, frame: &mut Frame, state: &InputState) {
    let area = centered_rect(50, 3, frame.area());
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Yellow).bg(Color::Black));

    let mut spans = Vec::new();

    let cursor_position = state.cursor_position.min(state.buffer.len());
    let (left, right) = state.buffer.split_at(cursor_position);

    spans.push(Span::raw(left));
    spans.push(Span::styled(
        "█",
        Style::default().fg(Color::White).bg(Color::Black).bold(),
    ));
    if cursor_position == state.buffer.len() {
        spans.push(Span::raw(right));
    } else {
        spans.push(Span::raw(all_but_first(right)));
    }

    let input = Paragraph::new(Line::from(spans))
        .block(block)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left);

    frame.render_widget(Clear, area);
    frame.render_widget(input, area);
}

pub fn show_yes_no_modal(title: &str, frame: &mut Frame, selected: bool) {
    let area = centered_rect(40, 3, frame.area());

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Yellow).bg(Color::Black));

    let (yes_style, no_style) = if selected {
        (
            Style::default().fg(Color::Black).bg(Color::Green).bold(),
            Style::default().fg(Color::White),
        )
    } else {
        (
            Style::default().fg(Color::White),
            Style::default().fg(Color::Black).bg(Color::Red).bold(),
        )
    };

    let options_line = Line::from(vec![
        Span::styled(" [Yes] ", yes_style),
        Span::raw("  "),
        Span::styled(" [No] ", no_style),
    ])
    .centered();

    let paragraph = Paragraph::new(options_line)
        .block(block)
        .alignment(Alignment::Center);

    frame.render_widget(Clear, area);
    frame.render_widget(paragraph, area);
}
