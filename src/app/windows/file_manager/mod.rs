mod input;

use std::{path::Path, sync::mpsc::Sender};

use crossterm::event::Event;
use input::handle_event;
use ratatui::{
    Frame,
    layout::{Direction, Rect},
    style::{Color, Style},
    widgets::Block,
};

use crate::{
    app::{
        App, AppEvent, Error, InputMode, Result,
        widgets::{add_title_to_block, draw_entries_table},
        window::Window,
    },
    directory_entry::{DirectoryEntry, read_directory},
};

use super::Split;

#[derive(Clone)]
pub struct FileManagerWindow {
    directory: String,
    entries: Vec<DirectoryEntry>,
    selected_index: usize,
}

impl FileManagerWindow {
    pub fn new(directory: &str) -> Result<Self> {
        let path = Path::new(directory);

        if !path.is_dir() {
            return Err(Error::InvalidDirectoryPath(directory.into()));
        }

        Ok(Self {
            directory: directory.into(),
            entries: read_directory(path)?,
            selected_index: 0,
        })
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
}

impl Window for FileManagerWindow {
    fn render(&self, _: &App, frame: &mut Frame, area: Rect, focused: bool) {
        let mut block = Block::bordered();

        if focused {
            block = block.border_style(Style::default().fg(Color::Cyan));
        }

        block = add_title_to_block(&self.directory, block);

        draw_entries_table(frame, area, &self.entries, self.selected_index, block);
    }

    fn handle_event(
        &mut self,
        input_mode: &InputMode,
        event: &Event,
        focused: bool,
        event_tx: &Sender<AppEvent>,
        handled: bool,
    ) -> bool {
        if !focused || handled {
            return false;
        }

        handle_event(self, input_mode, event, event_tx)
    }

    fn reset(&mut self) -> Result<()> {
        self.entries = read_directory(Path::new(&self.directory))?;
        self.selected_index = self.selected_index.min(self.entries.len() - 1);

        Ok(())
    }

    fn split(self: Box<Self>, direction: Direction) -> Box<dyn Window> {
        Box::new(Split::new(
            direction,
            vec![
                Box::new(FileManagerWindow {
                    directory: self.directory.clone(),
                    entries: self.entries.clone(),
                    selected_index: 0,
                }),
                self,
            ],
        ))
    }
}
