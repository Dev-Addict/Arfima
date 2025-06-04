mod error;

use std::path::Path;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::Constraint,
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Cell, Row, Table, TableState},
};

use crate::directory_entry::{DirectoryEntry, read_directory};
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
        let title = Line::from("Arfima").bold().blue().centered();

        let instructions = Line::from(vec![
            " Up ".into(),
            "<K>".blue().bold(),
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
                Row::new(vec![
                    Cell::from(entry.icon()),
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
            _ => {}
        }
    }

    fn quit(&mut self) {
        self.running = false;
    }
}
