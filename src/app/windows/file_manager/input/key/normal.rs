use std::{path::Path, sync::mpsc::Sender};

use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    app::{
        AppEvent, InputMode, precommand::Precommand, widgets::types::InputState,
        windows::FileManagerWindow,
    },
    directory_entry::DirectoryEntryType,
    utils::file::{FileError, get_opening_methods, open_file},
};

pub fn handle(
    window: &mut FileManagerWindow,
    input_mode: &InputMode,
    key: &KeyEvent,
    event_tx: &Sender<AppEvent>,
) -> bool {
    if let InputMode::Normal { precommand } = input_mode {
        match (key.modifiers, key.code) {
            (_, KeyCode::Down | KeyCode::Char('j')) => {
                let mut count = 1;
                if let Some(Precommand::Repeat(repeat)) = precommand {
                    count = *repeat;
                }

                let _ = event_tx.send(AppEvent::UpdatePrecommand(None));

                window.selected_index = window
                    .selected_index
                    .saturating_add(count)
                    .min(window.entries.len().saturating_sub(1));
            }
            (_, KeyCode::Up | KeyCode::Char('k')) => {
                let mut count = 1;
                if let Some(Precommand::Repeat(repeat)) = precommand {
                    count = *repeat;
                }

                let _ = event_tx.send(AppEvent::UpdatePrecommand(None));

                window.selected_index = window.selected_index.saturating_sub(count);
            }
            (_, KeyCode::Left | KeyCode::Char('h') | KeyCode::Backspace) => {
                let mut count = 0;
                if let Some(Precommand::Repeat(repeat)) = precommand {
                    if *repeat == 0 {
                        return true;
                    }

                    count = repeat.saturating_sub(1);
                }

                let mut target_directory: &Path;

                if let Some(parent) = Path::new(&window.directory).parent() {
                    target_directory = parent;
                } else {
                    return true;
                }

                while let Some(parent) = Path::new(target_directory).parent() {
                    if count == 0 {
                        break;
                    }

                    target_directory = parent;
                    count = count.saturating_sub(1);
                }

                if let Err(e) = window.set_directory(target_directory.to_string_lossy().to_string())
                {
                    let _ = event_tx.send(AppEvent::SetError(Some(e)));
                }
            }
            (_, KeyCode::Right | KeyCode::Char('l') | KeyCode::Enter) => {
                if let Some(entry) = window.entries.get(window.selected_index) {
                    match entry.entry_type() {
                        DirectoryEntryType::Directory => {
                            let _ =
                                window.set_directory(entry.path().to_string_lossy().to_string());
                        }
                        _ => {
                            let _ = open_file(entry.path());
                        }
                    }
                }
            }
            (_, KeyCode::Char('o')) => {
                if let Some(entry) = window.entries.get(window.selected_index) {
                    if *entry.entry_type() != DirectoryEntryType::Directory {
                        match get_opening_methods(entry.path()) {
                            Ok(apps) => {
                                if apps.is_empty() {
                                    let _ = event_tx.send(AppEvent::SetError(Some(
                                        FileError::NoAppsFound.into(),
                                    )));
                                } else {
                                    let _ = event_tx.send(AppEvent::UpdateInputMode(
                                        InputMode::Opening {
                                            apps,
                                            path: entry.path().to_string_lossy().to_string(),
                                            selected_index: 0,
                                        },
                                    ));
                                }
                            }
                            Err(e) => {
                                let _ = event_tx.send(AppEvent::SetError(Some(e.into())));
                            }
                        }
                    }
                }
            }
            (_, KeyCode::Char('a')) => {
                let _ = event_tx.send(AppEvent::UpdateInputMode(InputMode::Adding {
                    state: InputState::new(""),
                }));
            }
            (_, KeyCode::Char('r')) => {
                if let Some(entry) = window.entries.get(window.selected_index) {
                    let _ = event_tx.send(AppEvent::UpdateInputMode(InputMode::Renaming {
                        original: entry.name().into(),
                        state: InputState::new(entry.name()),
                    }));
                }
            }
            (_, KeyCode::Char('d')) => {
                if let Some(entry) = window.entries.get(window.selected_index) {
                    let _ = event_tx.send(AppEvent::UpdateInputMode(InputMode::Removing {
                        path: entry.path().to_string_lossy().to_string(),
                        removing_selected: false,
                    }));
                }
            }
            (_, KeyCode::Home | KeyCode::Char('g')) => {
                if let Some(Precommand::Repeat(repeat)) = precommand {
                    window.selected_index = (*repeat - 1).min(window.entries.len() - 1).max(1);

                    let _ = event_tx.send(AppEvent::UpdatePrecommand(None));
                } else {
                    window.selected_index = 0;
                }
            }
            (_, KeyCode::End | KeyCode::Char('G')) => {
                window.selected_index = window.entries.len().saturating_sub(1)
            }
            _ => {
                return false;
            }
        }

        return true;
    }

    false
}
