use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Color, Style, Stylize},
    text::Span,
    widgets::{Block, Cell, Row, Table, TableState},
};

use crate::directory_entry::DirectoryEntry;

pub fn draw_entries_table(
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

            let mut cells = vec![Cell::from(icon), Cell::from(entry.name())];

            if area.width >= 36 {
                cells.push(Cell::from(entry.formatted_size().unwrap_or_default()));
            }

            if area.width >= 54 {
                cells.push(Cell::from(entry.formatted_modified().unwrap_or_default()));
            }

            Row::new(cells)
        })
        .collect();

    let mut widths = vec![Constraint::Length(2), Constraint::Fill(1)];
    let mut headers = vec!["", "Name"];

    if area.width >= 36 {
        widths.push(Constraint::Min(6));
        headers.push("Size");
    }

    if area.width >= 54 {
        widths.push(Constraint::Min(18));
        headers.push("Modified");
    }

    let table = Table::new(rows, widths)
        .header(Row::new(headers).style(Style::default().fg(Color::Cyan).bold()))
        .row_highlight_style(Style::default().reversed().bold())
        .block(block);

    let mut state = TableState::default();
    state.select(Some(selected_index));

    frame.render_stateful_widget(table, area, &mut state);
}
