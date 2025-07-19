use ratatui::{
    Frame,
    layout::Constraint,
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Cell, Clear, Row, Table, TableState},
};

use crate::app::widgets::centered_rect::get_centered_rect;

pub fn show_opening_modal(file: &str, apps: &[String], frame: &mut Frame, selected_index: usize) {
    let area = get_centered_rect(
        40,
        (apps.len().saturating_add(2))
            .try_into()
            .unwrap_or(10)
            .min(10),
        frame.area(),
    );

    let block = Block::default()
        .title(format!("Opening {file}"))
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Yellow).bg(Color::Black));

    let rows: Vec<Row> = apps
        .iter()
        .map(|app| Row::new(vec![Cell::from(app.as_str())]))
        .collect();

    let table = Table::new(rows, vec![Constraint::Fill(1)])
        .row_highlight_style(Style::default().reversed().bold())
        .block(block);

    let mut state = TableState::default();
    state.select(Some(selected_index));

    frame.render_widget(Clear, area);
    frame.render_stateful_widget(table, area, &mut state);
}
