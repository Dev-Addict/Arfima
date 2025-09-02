use std::{
    any::{Any, TypeId},
    path::{Path, PathBuf},
};

use crossbeam::channel::Sender;
use crossterm::event::Event;
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
        window::{Window, WindowSize, generate_window_id},
    },
    config::Config,
    directory_entry::read_directory,
};

use super::{FileManagerWindow, SplitWindow, input::handle_event};

impl Window for FileManagerWindow {
    fn id(&self) -> u32 {
        self.id
    }

    fn render(&self, app: &App, frame: &mut Frame, area: Rect, focused: bool) {
        let mut block = Block::bordered();

        if focused {
            block = block.border_style(Style::default().fg(Color::Cyan));
        }

        block = add_title_to_block(&self.directory, block);

        draw_entries_table(
            frame,
            area,
            &self.entries,
            self.selected_index,
            block,
            &app.config,
        );
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

    fn reset(&mut self, _: &Config) -> Result<()> {
        self.entries = read_directory(Path::new(&self.directory))?;
        self.selected_index = self.selected_index.min(self.entries.len() - 1);

        Ok(())
    }

    fn split(self: Box<Self>, direction: Direction, count: usize) -> Box<dyn Window> {
        let mut windows: Vec<Box<dyn Window>> = Vec::with_capacity(count.max(1));

        for _ in 0..count {
            windows.push(Box::new(FileManagerWindow {
                id: generate_window_id(),
                directory: self.directory.clone(),
                entries: self.entries.clone(),
                selected_index: self.selected_index,
                window_size: WindowSize::Default,
            }));
        }

        Box::new(SplitWindow::with_window_size(
            direction,
            windows,
            self.window_size,
        ))
    }

    fn get_window_size(&self) -> &WindowSize {
        &self.window_size
    }

    fn adjust_window_size(
        &mut self,
        direction: Direction,
        adjustment: isize,
        parent: Option<(&Direction, usize)>,
    ) -> bool {
        if let Some((d, windows)) = parent {
            if d == &direction {
                self.window_size = match self.window_size {
                    WindowSize::Default => {
                        WindowSize::Adjusted(adjustment.saturating_mul(windows.cast_signed()))
                    }
                    WindowSize::DefaultSize(size) => WindowSize::AdjustedSize(
                        size,
                        adjustment.saturating_mul(windows.cast_signed()),
                    ),
                    WindowSize::Adjusted(prev) => WindowSize::Adjusted(
                        prev.saturating_add(adjustment.saturating_mul(windows.cast_signed())),
                    ),
                    WindowSize::AdjustedSize(size, prev) => WindowSize::AdjustedSize(
                        size,
                        prev.saturating_add(adjustment.saturating_mul(windows.cast_signed())),
                    ),
                };

                return true;
            }
        }

        false
    }

    fn includes(&self, id: u32) -> bool {
        self.id == id
    }

    fn open(self: Box<Self>, path: PathBuf, _: bool) -> (Box<dyn Window>, Option<Error>) {
        let path = Path::new(&path);

        if !path.is_dir() {
            return (
                self,
                Some(Error::InvalidDirectoryPath(
                    path.to_string_lossy().to_string(),
                )),
            );
        }

        let entries = match read_directory(path) {
            Ok(entries) => entries,
            Err(e) => return (self, Some(e.into())),
        };

        (
            Box::new(Self {
                id: self.id,
                directory: path.to_string_lossy().to_string(),
                entries,
                selected_index: 0,
                window_size: self.window_size,
            }),
            None,
        )
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn includes_type_id(&self, type_id: std::any::TypeId) -> Option<u32> {
        if type_id == TypeId::of::<FileManagerWindow>() {
            Some(self.id)
        } else {
            None
        }
    }
}
