mod error;

use std::path::Path;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    style::Stylize,
    text::Line,
    widgets::{Block, List},
};

use crate::directory_entry::{DirectoryEntry, read_directory};
use error::Error;

#[derive(Debug, Default)]
pub struct App {
    running: bool,
    directory: String,
    entries: Vec<DirectoryEntry>,
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

        let block = Block::default()
            .title(title.centered())
            .title_bottom(instructions.left_aligned());

        frame.render_widget(
            List::new(
                self.entries
                    .iter()
                    .map(|entry| entry.name().to_string())
                    .collect::<Vec<_>>(),
            )
            .block(block),
            frame.area(),
        );
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
            _ => {}
        }
    }

    fn quit(&mut self) {
        self.running = false;
    }
}
