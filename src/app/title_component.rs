use ratatui::{
    style::{Color, Style, Stylize},
    text::{Line, Span},
};

pub struct TitleComponent;

impl TitleComponent {
    pub fn get(directory: &str) -> Line {
        let mut title_items: Vec<Span> = vec![
            Span::styled("", Style::default()),
            Span::styled(" Arfima ", Style::default().reversed()),
            Span::styled("╱", Style::default().reversed().bg(Color::Reset)),
        ];

        for directory in directory[1..].split("/") {
            title_items.push(Span::styled(
                format!(" {} ", directory),
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
}
