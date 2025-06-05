use std::path::Path;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{directory_entry::DirectoryEntryType, utils::open_file};

use super::{App, InputMode};

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
                app.input_mode = InputMode::Adding { buffer: "".into() };
            }
            (_, KeyCode::Char('r')) => {
                if let Some(entry) = app.entries.get(app.selected_index) {
                    app.input_mode = InputMode::Renaming {
                        original: entry.name().into(),
                        buffer: entry.name().into(),
                    };
                }
            }
            _ => {}
        },
        InputMode::Adding { buffer } => match (key.modifiers, key.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => app.quit(),
            (_, KeyCode::Esc) => {
                app.input_mode = InputMode::Normal;
            }
            (_, KeyCode::Char(c)) => {
                buffer.push(c);
            }
            (_, KeyCode::Backspace) => {
                buffer.pop();
            }
            (_, KeyCode::Enter) => {
                if app.add_path().is_ok() {
                    app.input_mode = InputMode::Normal;
                }
            }
            _ => {}
        },
        InputMode::Renaming { buffer, .. } => match (key.modifiers, key.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => app.quit(),
            (_, KeyCode::Esc) => {
                app.input_mode = InputMode::Normal;
            }
            (_, KeyCode::Char(c)) => {
                buffer.push(c);
            }
            (_, KeyCode::Backspace) => {
                buffer.pop();
            }
            (_, KeyCode::Enter) => {
                if app.rename_path().is_ok() {
                    app.input_mode = InputMode::Normal;
                }
            }
            _ => {}
        },
        _ => {}
    }
}
