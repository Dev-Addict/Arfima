use ratatui::{
    Frame,
    layout::Constraint,
    style::{Color, Style, Stylize},
    text::Span,
    widgets::{Block, Cell, Row, Table, TableState},
};

use crate::app::App;

pub fn draw_entries_table(frame: &mut Frame, app: &App, block: Block) {
    let rows: Vec<Row> = app
        .entries
        .iter()
        .map(|entry| {
            let (icon, color) = entry.icon();

            let icon = match color {
                Some(color) => Span::styled(format!("{} ", icon), Style::default().fg(color)),
                None => Span::raw(icon),
            };

            Row::new(vec![
                Cell::from(icon),
                Cell::from(entry.name()),
                Cell::from(entry.formatted_size().unwrap_or_default()),
                Cell::from(entry.formatted_modified().unwrap_or_default()),
            ])
        })
        .collect();

    let widths = [
        Constraint::Length(2),
        Constraint::Fill(1),
        Constraint::Min(6),
        Constraint::Min(10),
    ];

    let table = Table::new(rows, widths)
        .header(
            Row::new(vec!["", "Name", "Size", "Modified"])
                .style(Style::default().fg(Color::Cyan).bold()),
        )
        .row_highlight_style(Style::default().reversed().bold())
        .block(block);

    let mut state = TableState::default();
    state.select(Some(app.selected_index));

    frame.render_stateful_widget(table, frame.area(), &mut state);
}
