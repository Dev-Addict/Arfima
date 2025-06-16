use std::cmp::max;

use ratatui::{
    layout::Constraint,
    style::{Color, Style, Stylize},
    widgets::{Cell, Row, Table},
};

use crate::{
    data::help::get_help,
    utils::{format_keybindings, str::wrap_text},
};

pub fn get_help_table<'a>(width: u16) -> Table<'a> {
    let description_width = width.saturating_sub(36);

    let help = get_help();

    let mut rows: Vec<Row> = Vec::new();

    for mode in help {
        let (height, description) = wrap_text(mode.description(), description_width);

        rows.push(
            Row::new(vec![
                Cell::from(format!("{}", mode.mode()))
                    .style(Style::default().fg(Color::Green).bold()),
                Cell::from(""),
                Cell::from(description),
            ])
            .height(height.try_into().unwrap_or(1)),
        );

        for item in mode.items() {
            let (description_height, description) =
                wrap_text(item.description(), description_width);
            let (keybindings_height, keybindings) =
                wrap_text(&format_keybindings(item.keys(), item.count()), 16);

            rows.push(
                Row::new(vec![
                    Cell::from(item.name()),
                    Cell::from(keybindings).style(Style::default().fg(Color::Blue).bold()),
                    Cell::from(description),
                ])
                .height(
                    max(description_height, keybindings_height)
                        .try_into()
                        .unwrap_or(1),
                ),
            );
        }
    }

    let widths = [
        Constraint::Length(16),
        Constraint::Length(16),
        Constraint::Fill(1),
    ];

    Table::new(rows, widths).row_highlight_style(Style::default().reversed().bold())
}
