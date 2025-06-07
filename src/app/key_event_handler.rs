use std::path::Path;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{directory_entry::DirectoryEntryType, utils::open_file};

use super::{App, InputMode, InputState, help::get_help_enteries_len};

pub fn handle_key_event(app: &mut App, key: KeyEvent) {
    match &mut app.input_mode {
        InputMode::Normal => match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => app.quit(),
            (_, KeyCode::Down | KeyCode::Char('j')) => {
                if app.selected_index + 1 < app.entries.len() {
                    app.selected_index += 1;
                }
            }
            (_, KeyCode::Up | KeyCode::Char('k')) => {
                if app.selected_index > 0 {
                    app.selected_index -= 1;
                }
            }
            (KeyModifiers::CONTROL, KeyCode::Char('h')) => {
                app.input_mode = InputMode::Help { selected_index: 0 };
            }
            (_, KeyCode::Left | KeyCode::Char('h') | KeyCode::Backspace) => {
                if let Some(parent) = Path::new(&app.directory).parent() {
                    let _ = app.set_directory(parent.to_string_lossy().to_string());
                }
            }
            (_, KeyCode::Right | KeyCode::Char('l') | KeyCode::Enter) => {
                if let Some(entry) = app.entries.get(app.selected_index) {
                    match entry.entry_type() {
                        DirectoryEntryType::Directory => {
                            let _ = app.set_directory(entry.path().to_string_lossy().to_string());
                        }
                        _ => {
                            let _ = open_file(entry.path());
                        }
                    }
                }
            }
            (_, KeyCode::Char('a')) => {
                app.input_mode = InputMode::Adding {
                    state: InputState {
                        buffer: "".into(),
                        cursor_position: 0,
                    },
                };
            }
            (_, KeyCode::Char('r')) => {
                if let Some(entry) = app.entries.get(app.selected_index) {
                    app.input_mode = InputMode::Renaming {
                        original: entry.name().into(),
                        state: InputState {
                            buffer: entry.name().into(),
                            cursor_position: entry.name().len(),
                        },
                    };
                }
            }
            (_, KeyCode::Char('d')) => {
                if let Some(entry) = app.entries.get(app.selected_index) {
                    app.input_mode = InputMode::Removing {
                        path: entry.path().to_string_lossy().to_string(),
                    };
                }
            }
            (_, KeyCode::Home | KeyCode::Char('g')) => app.selected_index = 0,
            (_, KeyCode::End | KeyCode::Char('G')) => {
                app.selected_index = app.entries.len().saturating_sub(1)
            }
            _ => {}
        },
        InputMode::Adding { state } => match (key.modifiers, key.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => app.quit(),
            (_, KeyCode::Esc) => {
                app.input_mode = InputMode::Normal;
            }
            (_, KeyCode::Char(c)) => {
                state.buffer.insert(state.cursor_position, c);
                state.cursor_position += 1;
            }
            (_, KeyCode::Backspace) => {
                if state.cursor_position > 0 {
                    state.buffer.remove(state.cursor_position - 1);
                    state.cursor_position -= 1;
                }
            }
            (_, KeyCode::Left) => state.cursor_position = state.cursor_position.saturating_sub(1),
            (_, KeyCode::Right) => {
                state.cursor_position = (state.cursor_position + 1).min(state.buffer.len());
            }
            (_, KeyCode::Home) => state.cursor_position = 0,
            (_, KeyCode::End) => state.cursor_position = state.buffer.len(),
            (_, KeyCode::Enter) => {
                if app.add_path().is_ok() {
                    app.input_mode = InputMode::Normal;
                }
            }
            _ => {}
        },
        InputMode::Renaming { state, .. } => match (key.modifiers, key.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => app.quit(),
            (_, KeyCode::Esc) => {
                app.input_mode = InputMode::Normal;
            }
            (_, KeyCode::Char(c)) => {
                state.buffer.insert(state.cursor_position, c);
                state.cursor_position += 1;
            }
            (_, KeyCode::Backspace) => {
                if state.cursor_position > 0 {
                    state.buffer.remove(state.cursor_position - 1);
                    state.cursor_position -= 1;
                }
            }
            (_, KeyCode::Left) => state.cursor_position = state.cursor_position.saturating_sub(1),
            (_, KeyCode::Right) => {
                state.cursor_position = (state.cursor_position + 1).min(state.buffer.len());
            }
            (_, KeyCode::Home) => state.cursor_position = 0,
            (_, KeyCode::End) => state.cursor_position = state.buffer.len(),
            (_, KeyCode::Enter) => {
                if app.rename_path().is_ok() {
                    app.input_mode = InputMode::Normal;
                }
            }
            _ => {}
        },
        InputMode::Removing { .. } => match (key.modifiers, key.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => app.quit(),
            (_, KeyCode::Esc | KeyCode::Char('n') | KeyCode::Char('N')) => {
                app.input_mode = InputMode::Normal;
                app.removing_selected = false;
            }
            (_, KeyCode::Char('y') | KeyCode::Char('Y')) => {
                let _ = app.delete_path();
                app.input_mode = InputMode::Normal;
                app.removing_selected = false;
            }
            (_, KeyCode::Char('l') | KeyCode::Right) => {
                app.removing_selected = false;
            }
            (_, KeyCode::Char('h') | KeyCode::Left) => {
                app.removing_selected = true;
            }
            (_, KeyCode::Enter) => {
                if app.removing_selected {
                    let _ = app.delete_path();
                }

                app.input_mode = InputMode::Normal;
                app.removing_selected = false;
            }
            _ => {}
        },
        InputMode::Help { selected_index } => match (key.modifiers, key.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => app.quit(),
            (_, KeyCode::Esc | KeyCode::Char('q')) => {
                app.input_mode = InputMode::Normal;
            }
            (_, KeyCode::Down | KeyCode::Char('j')) => {
                if *selected_index + 1 < get_help_enteries_len() {
                    *selected_index = selected_index.saturating_add(1);
                }
            }
            (_, KeyCode::Up | KeyCode::Char('k')) => {
                if *selected_index > 0 {
                    *selected_index = selected_index.saturating_sub(1);
                }
            }
            (_, KeyCode::Home | KeyCode::Char('g')) => *selected_index = 0,
            (_, KeyCode::End | KeyCode::Char('G')) => {
                *selected_index = get_help_enteries_len().saturating_sub(1)
            }
            _ => {}
        },
    }
}
