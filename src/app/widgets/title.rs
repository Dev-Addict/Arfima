use ratatui::{
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, block::Title},
};

pub fn get_title<'a>(path: String) -> impl Into<Title<'a>> {
    let mut title_items: Vec<Span> = vec![
        Span::styled("", Style::default()),
        Span::styled(" Arfima ", Style::default().reversed()),
        Span::styled("╱", Style::default().reversed().bg(Color::Reset)),
    ];

    for directory in path[1..].split("/") {
        title_items.push(Span::styled(
            format!(" {directory} "),
            Style::default().reversed(),
        ));
        title_items.push(Span::styled(
            "╱",
            Style::default().reversed().bg(Color::Reset),
        ));
    }

    title_items.pop();
    title_items.push(Span::styled("", Style::default()));

    Line::from(title_items).bold()
}

pub fn add_title_to_block<'a>(directory: &'a str, block: Block<'a>) -> Block<'a> {
    let mut title_items: Vec<Span> = vec![
        Span::styled("", Style::default()),
        Span::styled(" Arfima ", Style::default().reversed()),
        Span::styled("╱", Style::default().reversed().bg(Color::Reset)),
    ];

    for directory in directory[1..].split("/") {
        title_items.push(Span::styled(
            format!(" {directory} "),
            Style::default().reversed(),
        ));
        title_items.push(Span::styled(
            "╱",
            Style::default().reversed().bg(Color::Reset),
        ));
    }

    title_items.pop();
    title_items.push(Span::styled("", Style::default()));

    block.title(Line::from(title_items).bold())
}
