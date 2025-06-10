use std::path::Path;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    app::{App, InputMode, widgets::types::InputState},
    directory_entry::DirectoryEntryType,
    utils::open_file,
};

pub fn handle(app: &mut App, key: KeyEvent) {
    match (key.modifiers, key.code) {
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
                state: InputState::new("", 0),
            };
        }
        (_, KeyCode::Char('r')) => {
            if let Some(entry) = app.entries.get(app.selected_index) {
                app.input_mode = InputMode::Renaming {
                    original: entry.name().into(),
                    state: InputState::new(entry.name(), entry.name().len()),
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
    }
}
