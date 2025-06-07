use ratatui::{
    Frame,
    layout::Alignment,
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, TableState},
};

use crate::utils::{all_but_first, first_char_str};

use super::{
    InputState,
    centered_rect::{centered_rect, centered_rect_by_percent},
    components::HelpTableComponent,
};

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
    if cursor_position == state.buffer.len() {
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

pub fn show_help_modal(frame: &mut Frame, selected_index: usize) {
    let area = centered_rect_by_percent(80, 70, frame.area());

    let block = Block::default()
        .title("Help")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Yellow).bg(Color::Black));

    let table = HelpTableComponent::get(area.width).block(block);

    let mut state = TableState::default();
    state.select(Some(selected_index));

    frame.render_widget(Clear, area);
    frame.render_stateful_widget(table, area, &mut state);
}
