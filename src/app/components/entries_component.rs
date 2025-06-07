use ratatui::{
    layout::Constraint,
    style::{Color, Style, Stylize},
    text::Span,
    widgets::{Cell, Row, Table},
};

use crate::{directory_entry::DirectoryEntry, utils::hex_to_color};

pub struct EntriesComponent;

impl EntriesComponent {
    pub fn get(entries: &[DirectoryEntry]) -> Table {
        let rows: Vec<Row> = entries
            .iter()
            .map(|entry| {
                let (icon, color) = entry.icon();

                let icon = match color {
                    Some(color) => match hex_to_color(color) {
                        Some(color) => {
                            Span::styled(format!("{} ", icon), Style::default().fg(color))
                        }
                        None => Span::raw(icon),
                    },
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

        Table::new(rows, widths)
            .header(
                Row::new(vec!["", "Name", "Size", "Modified"])
                    .style(Style::default().fg(Color::Cyan).bold()),
            )
            .row_highlight_style(Style::default().reversed().bold())
    }
}
