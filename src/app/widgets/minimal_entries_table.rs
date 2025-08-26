use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Style, Stylize},
    text::Span,
    widgets::{Block, Cell, Row, Table, TableState},
};

use crate::directory_entry::DirectoryEntry;

pub fn draw_minimal_entries_table(
    frame: &mut Frame,
    area: Rect,
    entries: &[DirectoryEntry],
    selected_index: usize,
    block: Block,
) {
    let rows: Vec<Row> = entries
        .iter()
        .map(|entry| {
            let (icon, color) = entry.icon();

            let icon = match color {
                Some(color) => Span::styled(format!("{icon} "), Style::default().fg(color)),
                None => Span::raw(icon),
            };

            Row::new(vec![Cell::from(icon), Cell::from(entry.name())])
        })
        .collect();

    let table = Table::new(rows, vec![Constraint::Length(2), Constraint::Fill(1)])
        .row_highlight_style(Style::default().reversed().bold())
        .block(block);

    let mut state = TableState::default();
    state.select(Some(selected_index));

    frame.render_stateful_widget(table, area, &mut state);
}
