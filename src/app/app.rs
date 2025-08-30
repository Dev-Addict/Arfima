use std::sync::mpsc::{Receiver, Sender, channel};

use crossterm::event::{self};
use ratatui::{DefaultTerminal, layout::Direction};

use crate::{
    config::Config,
    directory_entry::DirectoryEntryType,
    types::CircularBuffer,
    utils::file::{FileError, get_opening_methods, open_file},
};

use super::{
    AppEvent, Error, InputMode, Result,
    input::handle_event,
    ui::render_ui,
    window::Window,
    windows::{DummyWindow, FileManagerWindow},
};

// TODO: Add command history buffer size to config

pub struct App {
    running: bool,
    pub input_mode: InputMode,
    pub error: Option<Error>,
    pub window: Box<dyn Window>,
    pub config: Config,
    pub command_history: CircularBuffer<String>,
    event_rx: Receiver<AppEvent>,
}

impl App {
    pub fn new(directory: &str, config: Config) -> Result<(Self, Sender<AppEvent>)> {
        let (tx, rx) = channel();

        Ok((
            Self {
                running: false,
                input_mode: InputMode::Normal { precommand: None },
                error: None,
                window: Box::new(FileManagerWindow::new(directory)?),
                config,
                command_history: CircularBuffer::new(50),
                event_rx: rx,
            },
            tx,
        ))
    }

    pub fn reset(&mut self) -> Result<()> {
        self.running = true;
        self.input_mode = InputMode::Normal { precommand: None };
        self.error = None;
        self.window.reset(&self.config)?;

        Ok(())
    }

    pub fn run(mut self, mut terminal: DefaultTerminal, event_tx: &Sender<AppEvent>) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| render_ui(&mut self, frame))?;

            self.handle_crossterm_events(event_tx)?;

            while let Ok(event) = self.event_rx.try_recv() {
                self.handle_app_events(event)?;
            }
        }
        Ok(())
    }

    fn handle_crossterm_events(&mut self, event_tx: &Sender<AppEvent>) -> Result<()> {
        let event = event::read()?;

        let handled = handle_event(self, &event);
        self.window
            .handle_event(&self.input_mode, &event, true, event_tx, handled);

        Ok(())
    }

    fn handle_app_events(&mut self, event: AppEvent) -> Result<()> {
        match event {
            AppEvent::UpdatePrecommand(new_precommand) => {
                if let InputMode::Normal { precommand } = &mut self.input_mode {
                    *precommand = new_precommand
                }
            }
            AppEvent::SetError(e) => self.error = e,
            AppEvent::UpdateInputMode(input_mode) => self.input_mode = input_mode,
            AppEvent::Open {
                path,
                new,
                entry_type,
            } => match entry_type {
                DirectoryEntryType::Directory => {
                    let window = std::mem::replace(&mut self.window, Box::new(DummyWindow));

                    (self.window, self.error) = window.open(path, new);
                }
                _ => {
                    if new {
                        match get_opening_methods(&path) {
                            Ok(apps) => {
                                if apps.is_empty() {
                                    self.error = Some(FileError::NoAppsFound.into());
                                } else {
                                    self.input_mode = InputMode::Opening {
                                        apps,
                                        path: path.to_string_lossy().to_string(),
                                        selected_index: 0,
                                    };
                                }
                            }
                            Err(e) => {
                                self.error = Some(e.into());
                            }
                        }
                    } else {
                        let _ = open_file(&path);
                    }
                }
            },
            AppEvent::Reset => self.reset()?,
        }

        Ok(())
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn next_window(&mut self) {
        if !self.window.next_window() {
            self.window.abs_next_window();
        }
    }

    pub fn prev_window(&mut self) {
        if !self.window.prev_window() {
            self.window.abs_prev_window();
        }
    }

    pub fn quit_focused_window(&mut self) {
        let window = std::mem::replace(&mut self.window, Box::new(DummyWindow));

        if let Some(window) = window.quit_focused_window() {
            self.window = window;
        } else {
            self.quit();
        }
    }

    pub fn adjust_window_size(&mut self, direction: Direction, adjustment: isize) {
        self.window.adjust_window_size(direction, adjustment, None);
    }
}
