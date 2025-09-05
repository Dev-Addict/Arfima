use ratatui::{
    layout::Alignment,
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::{
    app::widgets::types::{InputState, InputStateMode},
    utils::str::{all_but_first, first_char_str},
};

pub fn get_input(state: &InputState) -> Paragraph {
    let mut spans = Vec::new();

    let cursor_position = state.cursor_position().min(state.buffer().len());
    let parts = state.buffer_split();
    let mut i = 1;
    let mut cursor_pushed = false;

    spans.push(Span::raw(parts[0]));

    if let InputStateMode::Select { origin } = state.mode() {
        if *origin > cursor_position {
            if cursor_position == state.buffer().len() {
                spans.push(Span::styled(
                    "█",
                    Style::default().fg(Color::White).bg(Color::Black).bold(),
                ));
            } else {
                spans.push(Span::styled(
                    first_char_str(parts[1]).unwrap_or("█"),
                    Style::default().fg(Color::DarkGray).bg(Color::White).bold(),
                ));
            }

            spans.push(Span::styled(
                all_but_first(parts[1]),
                Style::default().bg(Color::Gray),
            ));
            cursor_pushed = true;
        } else {
            spans.push(Span::styled(parts[1], Style::default().bg(Color::Gray)));
        }
        i += 1;
    }

    if cursor_pushed {
        spans.push(Span::raw(parts[i]));
    } else if cursor_position == state.buffer().len() {
        spans.push(Span::styled(
            "█",
            Style::default().fg(Color::White).bg(Color::Black).bold(),
        ));
        spans.push(Span::raw(parts[i]));
    } else {
        spans.push(Span::styled(
            first_char_str(parts[i]).unwrap_or("█"),
            Style::default().fg(Color::DarkGray).bg(Color::White).bold(),
        ));
        spans.push(Span::raw(all_but_first(parts[i])));
    }

    Paragraph::new(Line::from(spans))
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left)
}
