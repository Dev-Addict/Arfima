mod entries_component;
mod error;
mod instructions_component;
mod title_component;

use std::path::Path;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use entries_component::EnteriesComponent;
use instructions_component::InstructionsComponent;
use ratatui::{
    DefaultTerminal, Frame,
    widgets::{Block, TableState},
};
use title_component::TitleComponent;

use crate::{
    directory_entry::{DirectoryEntry, DirectoryEntryType, read_directory},
    utils::open_file,
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
        let block = Block::bordered()
            .title(TitleComponent::get(&self.directory))
            .title_bottom(InstructionsComponent::get());

        let table = EnteriesComponent::get(&self.entries).block(block);

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
                            let _ = open_file(entry.path());
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
