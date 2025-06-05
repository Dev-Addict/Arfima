mod centered_rect;
mod components;
mod error;
mod key_event_handler;
mod show_modal;

use std::{fs, path::Path};

use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    widgets::{Block, TableState},
};

use crate::directory_entry::{DirectoryEntry, DirectoryEntryType, read_directory};
use centered_rect::centered_rect;
use components::{EntriesComponent, InstructionsComponent, TitleComponent};
use error::Error;
use key_event_handler::handle_key_event;
use show_modal::{show_input_modal, show_yes_no_modal};

#[derive(Debug)]
pub enum InputMode {
    Normal,
    Adding { buffer: String },
    Renaming { original: String, buffer: String },
    Removing { path: String },
}

#[derive(Debug)]
pub struct App {
    running: bool,
    directory: String,
    entries: Vec<DirectoryEntry>,
    selected_index: usize,
    input_mode: InputMode,
    removing_selected: bool,
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
            input_mode: InputMode::Normal,
            removing_selected: false,
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

        let table = EntriesComponent::get(&self.entries).block(block);

        let mut state = TableState::default();
        state.select(Some(self.selected_index));

        frame.render_stateful_widget(table, frame.area(), &mut state);

        match &self.input_mode {
            InputMode::Adding { buffer } => {
                show_input_modal("Add directory/file", frame, buffer);
            }
            InputMode::Renaming { buffer, .. } => {
                show_input_modal("Rename directory/file", frame, buffer);
            }
            InputMode::Removing { .. } => {
                show_yes_no_modal(
                    "Are you sure you want to delete directory/file?",
                    frame,
                    self.removing_selected,
                );
            }
            _ => {}
        }

        if let InputMode::Adding { buffer } = &self.input_mode {
            show_input_modal("Add directory/file", frame, buffer);
        }
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
        handle_key_event(self, key);
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
        self.selected_index = 0;

        Ok(())
    }

    pub fn add_path(&mut self) -> Result<()> {
        if let InputMode::Adding { buffer } = &mut self.input_mode {
            let new_path = Path::new(&self.directory).join(buffer);

            if new_path.extension().is_some() {
                if let Some(parent) = new_path.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::File::create(&new_path)?;
            } else {
                fs::create_dir_all(&new_path)?;
            }

            self.entries = read_directory(Path::new(&self.directory))?;
            return Ok(());
        }

        Err(Error::IncorrentInputMode)
    }

    pub fn rename_path(&mut self) -> Result<()> {
        if let InputMode::Renaming { original, buffer } = &mut self.input_mode {
            let new_path = Path::new(&self.directory).join(buffer);
            let original_path = Path::new(&self.directory).join(original);

            if new_path.is_dir() != original_path.is_dir() {
                return Err(Error::RenameBufferTypeMismatch);
            }

            if let Some(parent) = new_path.parent() {
                fs::create_dir_all(parent)?;
            }

            fs::rename(original_path, new_path)?;

            self.entries = read_directory(Path::new(&self.directory))?;
            return Ok(());
        }

        Err(Error::IncorrentInputMode)
    }

    pub fn delete_path(&mut self) -> Result<()> {
        if let InputMode::Removing { path } = &mut self.input_mode {
            if let Some(entry) = self.entries.get(self.selected_index) {
                match entry.entry_type() {
                    DirectoryEntryType::Directory => fs::remove_dir_all(path)?,
                    _ => fs::remove_file(path)?,
                }
            }

            self.entries = read_directory(Path::new(&self.directory))?;
            return Ok(());
        }

        Err(Error::IncorrentInputMode)
    }
}
