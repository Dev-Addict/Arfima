mod error;
mod input;
mod input_mode;
mod precommand;
mod result;
mod ui;
mod widgets;
mod window;

use std::{
    path::Path,
    sync::mpsc::{Receiver, Sender, channel},
};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
pub use error::Error;
use input::handle_key_event;
pub use input_mode::InputMode;
use precommand::Precommand;
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Direction, Rect},
    style::{Color, Style},
    widgets::Block,
};
pub use result::Result;
use ui::render_ui;
use widgets::{add_title_to_block, draw_entries_table, types::InputState};
use window::{Split, Window};

use crate::{
    directory_entry::{DirectoryEntry, DirectoryEntryType, read_directory},
    utils::open_file,
};

pub struct App {
    running: bool,
    input_mode: InputMode,
    removing_selected: bool,
    error: Option<Error>,
    window: Box<dyn Window>,
    event_rx: Receiver<AppEvent>,
}

pub enum AppEvent {
    UpdatePrecommand(Option<Precommand>),
    SetError(Option<Error>),
    UpdateInputMode(InputMode),
}

pub struct FileManagerWindow {
    directory: String,
    entries: Vec<DirectoryEntry>,
    selected_index: usize,
}

impl FileManagerWindow {
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

        block = add_title_to_block(self, block);

        draw_entries_table(frame, area, self, block);
    }

    fn handle_event(
        &mut self,
        event: &Event,
        focused: bool,
        precommand: Option<&Precommand>,
        event_tx: &Sender<AppEvent>,
    ) {
        if focused {
            if let Event::Key(key) = event {
                if key.kind == KeyEventKind::Press {
                    match (key.modifiers, key.code) {
                        (_, KeyCode::Down | KeyCode::Char('j')) => {
                            let mut count = 1;
                            if let Some(Precommand::Repeat(repeat)) = precommand {
                                count = *repeat;
                            }

                            let _ = event_tx.send(AppEvent::UpdatePrecommand(None));

                            self.selected_index = self
                                .selected_index
                                .saturating_add(count)
                                .min(self.entries.len());
                        }
                        (_, KeyCode::Up | KeyCode::Char('k')) => {
                            let mut count = 1;
                            if let Some(Precommand::Repeat(repeat)) = precommand {
                                count = *repeat;
                            }

                            let _ = event_tx.send(AppEvent::UpdatePrecommand(None));

                            self.selected_index = self.selected_index.saturating_sub(count);
                        }
                        (_, KeyCode::Left | KeyCode::Char('h') | KeyCode::Backspace) => {
                            let mut count = 0;
                            if let Some(Precommand::Repeat(repeat)) = precommand {
                                if *repeat == 0 {
                                    return;
                                }

                                count = repeat.saturating_sub(1);
                            }

                            let mut target_directory: &Path;

                            if let Some(parent) = Path::new(&self.directory).parent() {
                                target_directory = parent;
                            } else {
                                return;
                            }

                            while let Some(parent) = Path::new(target_directory).parent() {
                                if count == 0 {
                                    break;
                                }

                                target_directory = parent;
                                count = count.saturating_sub(1);
                            }

                            if let Err(e) =
                                self.set_directory(target_directory.to_string_lossy().to_string())
                            {
                                let _ = event_tx.send(AppEvent::SetError(Some(e)));
                            }
                        }
                        (_, KeyCode::Right | KeyCode::Char('l') | KeyCode::Enter) => {
                            if let Some(entry) = self.entries.get(self.selected_index) {
                                match entry.entry_type() {
                                    DirectoryEntryType::Directory => {
                                        let _ = self.set_directory(
                                            entry.path().to_string_lossy().to_string(),
                                        );
                                    }
                                    _ => {
                                        let _ = open_file(entry.path());
                                    }
                                }
                            }
                        }
                        (_, KeyCode::Char('a')) => {
                            let _ = event_tx.send(AppEvent::UpdateInputMode(InputMode::Adding {
                                state: InputState::new("", 0),
                            }));
                        }
                        (_, KeyCode::Char('r')) => {
                            if let Some(entry) = self.entries.get(self.selected_index) {
                                let _ =
                                    event_tx.send(AppEvent::UpdateInputMode(InputMode::Renaming {
                                        original: entry.name().into(),
                                        state: InputState::new(entry.name(), entry.name().len()),
                                    }));
                            }
                        }
                        (_, KeyCode::Char('d')) => {
                            if let Some(entry) = self.entries.get(self.selected_index) {
                                let _ =
                                    event_tx.send(AppEvent::UpdateInputMode(InputMode::Removing {
                                        path: entry.path().to_string_lossy().to_string(),
                                    }));
                            }
                        }
                        (_, KeyCode::Home | KeyCode::Char('g')) => self.selected_index = 0,
                        (_, KeyCode::End | KeyCode::Char('G')) => {
                            self.selected_index = self.entries.len().saturating_sub(1)
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    fn reset(&mut self) -> Result<()> {
        self.entries = read_directory(Path::new(&self.directory))?;
        self.selected_index = self.selected_index.min(self.entries.len() - 1);

        Ok(())
    }
}

impl App {
    pub fn new(directory: &str) -> Result<(Self, Sender<AppEvent>)> {
        let path = Path::new(directory);

        if !path.is_dir() {
            return Err(Error::InvalidDirectoryPath(directory.into()));
        }

        let (tx, rx) = channel();

        Ok((
            Self {
                running: false,
                input_mode: InputMode::Normal { precommand: None },
                removing_selected: false,
                error: None,
                window: Box::new(Split::new(
                    Direction::Horizontal,
                    vec![
                        Box::new(FileManagerWindow {
                            directory: directory.into(),
                            entries: read_directory(path)?,
                            selected_index: 0,
                        }),
                        Box::new(FileManagerWindow {
                            directory: directory.into(),
                            entries: read_directory(path)?,
                            selected_index: 0,
                        }),
                    ],
                )),
                event_rx: rx,
            },
            tx,
        ))
    }

    pub fn reset(&mut self) -> Result<()> {
        self.running = true;
        self.input_mode = InputMode::Normal { precommand: None };
        self.removing_selected = false;
        self.error = None;

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

        match event {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }

        let precommand = match &self.input_mode {
            InputMode::Normal { precommand } => precommand.as_ref(),
            _ => None,
        };

        self.window.handle_event(&event, true, precommand, event_tx);

        Ok(())
    }

    fn handle_app_events(&mut self, event: AppEvent) -> Result<()> {
        match event {
            AppEvent::UpdatePrecommand(new_precommand) => {
                if let InputMode::Normal { precommand } = &mut self.input_mode {
                    *precommand = new_precommand;
                }
            }
            AppEvent::SetError(e) => {
                self.error = e;
            }
            AppEvent::UpdateInputMode(input_mode) => {
                self.input_mode = input_mode;
            }
        }

        Ok(())
    }

    fn on_key_event(&mut self, key: KeyEvent) {
        handle_key_event(self, key);
    }

    fn quit(&mut self) {
        self.running = false;
    }

    pub fn add_path(&mut self) -> Result<()> {
        // if let InputMode::Adding { state } = &mut self.input_mode {
        //     let new_path = Path::new(&self.directory).join(state.buffer());
        //
        //     if new_path.extension().is_some() {
        //         if let Some(parent) = new_path.parent() {
        //             fs::create_dir_all(parent)?;
        //         }
        //         fs::File::create(&new_path)?;
        //     } else {
        //         fs::create_dir_all(&new_path)?;
        //     }
        //
        //     self.entries = read_directory(Path::new(&self.directory))?;
        //     return Ok(());
        // }

        Err(Error::IncorrentInputMode)
    }

    pub fn rename_path(&mut self) -> Result<()> {
        // if let InputMode::Renaming { original, state } = &mut self.input_mode {
        //     let new_path = Path::new(&self.directory).join(state.buffer());
        //     let original_path = Path::new(&self.directory).join(original);
        //
        //     if let Some(parent) = new_path.parent() {
        //         fs::create_dir_all(parent)?;
        //     }
        //
        //     fs::rename(original_path, new_path)?;
        //
        //     self.entries = read_directory(Path::new(&self.directory))?;
        //     return Ok(());
        // }

        Err(Error::IncorrentInputMode)
    }

    pub fn delete_path(&mut self) -> Result<()> {
        // if let InputMode::Removing { path } = &mut self.input_mode {
        //     if let Some(entry) = self.entries.get(self.selected_index) {
        //         match entry.entry_type() {
        //             DirectoryEntryType::Directory => fs::remove_dir_all(path)?,
        //             _ => fs::remove_file(path)?,
        //         }
        //     }
        //
        //     self.entries = read_directory(Path::new(&self.directory))?;
        //     return Ok(());
        // }

        Err(Error::IncorrentInputMode)
    }
}
