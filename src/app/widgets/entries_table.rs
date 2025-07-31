use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Color, Style, Stylize},
    text::Span,
    widgets::{Block, Cell, Row, Table, TableState},
};

use crate::{config::Config, directory_entry::DirectoryEntry};

pub fn draw_entries_table(
    frame: &mut Frame,
    area: Rect,
    entries: &[DirectoryEntry],
    selected_index: usize,
    block: Block,
    config: &Config,
) {
    let entries_len_digits = entries.len().checked_ilog10().unwrap_or(0) + 1;

    let rows: Vec<Row> = entries
        .iter()
        .enumerate()
        .map(|(i, entry)| {
            let (icon, color) = entry.icon();

            let icon = match color {
                Some(color) => Span::styled(format!("{icon} "), Style::default().fg(color)),
                None => Span::raw(icon),
            };

            let mut cells = vec![];

            if config.number().active() {
                cells.push(Cell::from(format!(
                    "{}{} ",
                    " ".repeat(
                        (entries_len_digits - ((i + 1).checked_ilog10().unwrap_or(0) + 1))
                            .try_into()
                            .unwrap_or(0)
                    ),
                    i + 1
                )))
            }

            cells.push(Cell::from(icon));
            cells.push(Cell::from(entry.name()));

            if area.width >= 36 {
                cells.push(Cell::from(entry.formatted_size().unwrap_or_default()));
            }

            if area.width >= 54 {
                cells.push(Cell::from(entry.formatted_modified().unwrap_or_default()));
            }

            Row::new(cells)
        })
        .collect();

    let mut widths = vec![];
    let mut headers = vec![];

    if config.number().active() {
        widths.push(Constraint::Length(
            (entries_len_digits + 1).try_into().unwrap_or(3),
        ));
        headers.push("");
    }

    widths.push(Constraint::Length(2));
    headers.push("");

    widths.push(Constraint::Fill(1));
    headers.push("Name");

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
