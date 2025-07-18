use ratatui::{
    Frame,
    layout::Alignment,
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};

use crate::app::widgets::centered_rect::get_centered_rect;

pub fn show_yes_no_modal(title: &str, frame: &mut Frame, selected: bool) {
    let area = get_centered_rect(40, 3, frame.area());

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
