use std::{path::Path, sync::mpsc::Sender};

use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::Block,
};

use crate::{
    app::{
        App, AppEvent, Error, InputMode, Result,
        precommand::Precommand,
        widgets::{add_title_to_block, draw_entries_table, types::InputState},
        window::Window,
    },
    directory_entry::{DirectoryEntry, DirectoryEntryType, read_directory},
    utils::file::{add_path, delete_path, open_file, rename_path},
};

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
    ) {
        if !focused {
            return;
        }

        if let Event::Key(key) = event {
            if key.kind != KeyEventKind::Press {
                return;
            }

            match input_mode {
                InputMode::Normal { precommand } => match (key.modifiers, key.code) {
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
                                    let _ = self
                                        .set_directory(entry.path().to_string_lossy().to_string());
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
                            let _ = event_tx.send(AppEvent::UpdateInputMode(InputMode::Renaming {
                                original: entry.name().into(),
                                state: InputState::new(entry.name(), entry.name().len()),
                            }));
                        }
                    }
                    (_, KeyCode::Char('d')) => {
                        if let Some(entry) = self.entries.get(self.selected_index) {
                            let _ = event_tx.send(AppEvent::UpdateInputMode(InputMode::Removing {
                                path: entry.path().to_string_lossy().to_string(),
                                removing_selected: false,
                            }));
                        }
                    }
                    (_, KeyCode::Home | KeyCode::Char('g')) => self.selected_index = 0,
                    (_, KeyCode::End | KeyCode::Char('G')) => {
                        self.selected_index = self.entries.len().saturating_sub(1)
                    }
                    _ => {}
                },
                InputMode::Adding { state } => {
                    if let (_, KeyCode::Enter) = (key.modifiers, key.code) {
                        match add_path(&self.directory, state, event_tx) {
                            Ok(_) => {
                                let _ =
                                    event_tx.send(AppEvent::UpdateInputMode(InputMode::Normal {
                                        precommand: None,
                                    }));
                                let _ = event_tx.send(AppEvent::SetError(None));
                            }
                            Err(e) => {
                                let _ = event_tx.send(AppEvent::SetError(Some(e)));
                            }
                        }
                    }
                }
                InputMode::Renaming { state, original } => {
                    if let (_, KeyCode::Enter) = (key.modifiers, key.code) {
                        match rename_path(&self.directory, state, original, event_tx) {
                            Ok(_) => {
                                let _ =
                                    event_tx.send(AppEvent::UpdateInputMode(InputMode::Normal {
                                        precommand: None,
                                    }));
                                let _ = event_tx.send(AppEvent::SetError(None));
                            }
                            Err(e) => {
                                let _ = event_tx.send(AppEvent::SetError(Some(e)));
                            }
                        }
                    }
                }
                InputMode::Removing {
                    path,
                    removing_selected,
                } => match (key.modifiers, key.code) {
                    (_, KeyCode::Enter) => {
                        if *removing_selected {
                            match delete_path(self.entries.get(self.selected_index), path, event_tx)
                            {
                                Ok(_) => {
                                    let _ = event_tx.send(AppEvent::UpdateInputMode(
                                        InputMode::Normal { precommand: None },
                                    ));
                                    let _ = event_tx.send(AppEvent::SetError(None));
                                }
                                Err(e) => {
                                    let _ = event_tx.send(AppEvent::SetError(Some(e)));
                                    return;
                                }
                            }
                        }

                        let _ = event_tx.send(AppEvent::UpdateInputMode(InputMode::Normal {
                            precommand: None,
                        }));
                        let _ = event_tx.send(AppEvent::SetError(None));
                    }
                    (_, KeyCode::Char('y') | KeyCode::Char('Y')) => {
                        match delete_path(self.entries.get(self.selected_index), path, event_tx) {
                            Ok(_) => {
                                let _ =
                                    event_tx.send(AppEvent::UpdateInputMode(InputMode::Normal {
                                        precommand: None,
                                    }));
                                let _ = event_tx.send(AppEvent::SetError(None));
                            }
                            Err(e) => {
                                let _ = event_tx.send(AppEvent::SetError(Some(e)));
                                return;
                            }
                        }

                        let _ = event_tx.send(AppEvent::UpdateInputMode(InputMode::Normal {
                            precommand: None,
                        }));
                        let _ = event_tx.send(AppEvent::SetError(None));
                    }
                    _ => (),
                },
                _ => {}
            }
        }
    }

    fn reset(&mut self) -> Result<()> {
        self.entries = read_directory(Path::new(&self.directory))?;
        self.selected_index = self.selected_index.min(self.entries.len() - 1);

        Ok(())
    }
}
