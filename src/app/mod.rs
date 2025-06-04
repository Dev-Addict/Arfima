mod error;

use std::{path::Path, process::Command};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::Constraint,
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Cell, Row, Table, TableState},
};

use crate::{
    directory_entry::{DirectoryEntry, DirectoryEntryType, read_directory},
    hex_to_color::hex_to_color,
};
use error::Error;

#[derive(Debug, Default)]
pub struct App {
    running: bool,
    directory: String,
    entries: Vec<DirectoryEntry>,
    selected_index: usize,
}

pub type Result<T> = std::result::Result<T, Error>;

impl App {
    pub fn new(directory: String) -> Result<Self> {
        let path = Path::new(&directory);

        if !path.is_dir() {
            return Err(Error::InvalidDirectoryPath(directory));
        }

        Ok(Self {
            running: false,
            entries: read_directory(path)?,
            directory,
            selected_index: 0,
        })
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let mut title_items: Vec<Span> = vec![
            Span::styled("", Style::default()),
            Span::styled(" root ", Style::default().reversed()),
            Span::styled("╱", Style::default().reversed().bg(Color::Reset)),
        ];

        for directory in self.directory[1..].split("/") {
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

        let title = Line::from(title_items).bold();

        let instructions = Line::from(vec![
            " Up ".into(),
            "<K>".blue().bold(),
            " Down ".into(),
            "<J>".blue().bold(),
            " Back ".into(),
            "<H>".blue().bold(),
            " Down ".into(),
            "<J>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.left_aligned());

        let rows: Vec<Row> = self
            .entries
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

        let table = Table::new(rows, widths)
            .header(
                Row::new(vec!["", "Name", "Size", "Modified"])
                    .style(Style::default().fg(Color::Cyan).bold()),
            )
            .block(block)
            .row_highlight_style(Style::default().reversed().bold());

        let mut state = TableState::default();
        state.select(Some(self.selected_index));

        frame.render_stateful_widget(table, frame.area(), &mut state);
    }

    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            (_, KeyCode::Down | KeyCode::Char('j')) => {
                if self.selected_index + 1 < self.entries.len() {
                    self.selected_index += 1;
                }
            }
            (_, KeyCode::Up | KeyCode::Char('k')) => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
            }
            (_, KeyCode::Left | KeyCode::Char('h') | KeyCode::Backspace) => {
                if let Some(parent) = Path::new(&self.directory).parent() {
                    let _ = self.set_directory(parent.to_string_lossy().to_string());
                }
            }
            (_, KeyCode::Right | KeyCode::Char('l') | KeyCode::Enter) => {
                if let Some(entry) = self.entries.get(self.selected_index) {
                    match entry.entry_type() {
                        DirectoryEntryType::Directory => {
                            let _ = self.set_directory(entry.path().to_string_lossy().to_string());
                        }
                        _ => {
                            #[cfg(target_os = "macos")]
                            let mut cmd = Command::new("open");

                            #[cfg(target_os = "linux")]
                            let mut cmd = Command::new("xdg-open");

                            #[cfg(target_os = "windows")]
                            let mut cmd = Command::new("cmd");

                            #[cfg(target_os = "windows")]
                            {
                                cmd.args(["/C", "start", "", path]);
                            }

                            #[cfg(not(target_os = "windows"))]
                            {
                                cmd.arg(entry.path());
                            }

                            let _ = cmd.status();
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn quit(&mut self) {
        self.running = false;
    }

    fn set_directory(&mut self, directory: String) -> Result<()> {
        let path = Path::new(&directory);

        if !path.is_dir() {
            return Err(Error::InvalidDirectoryPath(directory));
        }

        self.entries = read_directory(path)?;
        self.directory = directory;

        Ok(())
    }
}
