mod error;
mod input;
mod input_mode;
mod precommand;
mod result;
mod ui;
mod widgets;
mod window;

use std::{fs, path::Path};

use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
pub use error::Error;
use input::handle_key_event;
pub use input_mode::InputMode;
use ratatui::{DefaultTerminal, Frame, layout::Rect};
pub use result::Result;
use ui::render_ui;
use widgets::get_title;
use window::{DefaultWindow, Window};

use crate::directory_entry::{DirectoryEntry, DirectoryEntryType, read_directory};

pub struct App {
    running: bool,
    directory: String,
    entries: Vec<DirectoryEntry>,
    selected_index: usize,
    input_mode: InputMode,
    removing_selected: bool,
    error: Option<Error>,
    windows: Box<dyn Window>,
}

impl App {
    pub fn new(directory: &str) -> Result<Self> {
        let path = Path::new(directory);

        if !path.is_dir() {
            return Err(Error::InvalidDirectoryPath(directory.into()));
        }

        Ok(Self {
            running: false,
            directory: directory.into(),
            entries: read_directory(path)?,
            selected_index: 0,
            input_mode: InputMode::Normal { precommand: None },
            removing_selected: false,
            error: None,
            windows: Box::new(DefaultWindow::new(|_: &mut Frame, _: Rect| {})),
        })
    }

    pub fn reset(&mut self) -> Result<()> {
        self.running = true;
        self.entries = read_directory(Path::new(&self.directory))?;
        self.input_mode = InputMode::Normal { precommand: None };
        self.removing_selected = false;
        self.error = None;

        Ok(())
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| render_ui(&mut self, frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
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
        if let InputMode::Adding { state } = &mut self.input_mode {
            let new_path = Path::new(&self.directory).join(state.buffer());

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
        if let InputMode::Renaming { original, state } = &mut self.input_mode {
            let new_path = Path::new(&self.directory).join(state.buffer());
            let original_path = Path::new(&self.directory).join(original);

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
