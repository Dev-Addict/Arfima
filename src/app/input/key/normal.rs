use std::path::Path;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    app::{App, InputMode, precommand::Precommand, widgets::types::InputState},
    directory_entry::DirectoryEntryType,
    utils::open_file,
};

pub fn handle(app: &mut App, key: KeyEvent) {
    if let InputMode::Normal { precommand } = &mut app.input_mode {
        match (key.modifiers, key.code) {
            (_, KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => app.quit(),
            (_, KeyCode::Esc) => {
                if let Err(e) = app.reset() {
                    app.error = Some(e);
                }
            }
            (_, KeyCode::Down | KeyCode::Char('j')) => {
                let mut count = 1;
                if let Some(Precommand::Repeat(repeat)) = precommand {
                    count = *repeat;
                }

                *precommand = None;

                app.selected_index = app
                    .selected_index
                    .saturating_add(count)
                    .min(app.entries.len());
            }
            (_, KeyCode::Up | KeyCode::Char('k')) => {
                let mut count = 1;
                if let Some(Precommand::Repeat(repeat)) = precommand {
                    count = *repeat;
                }

                *precommand = None;

                app.selected_index = app.selected_index.saturating_sub(count);
            }
            (KeyModifiers::CONTROL, KeyCode::Char('h')) => {
                app.input_mode = InputMode::Help { selected_index: 0 };
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

                if let Some(parent) = Path::new(&app.directory).parent() {
                    target_directory = parent;
                } else {
                    return;
                }

                while let Some(parent) = Path::new(target_directory).parent() {
                    target_directory = parent;
                    count = count.saturating_sub(1);

                    if count == 0 {
                        break;
                    }
                }

                if let Err(e) = app.set_directory(target_directory.to_string_lossy().to_string()) {
                    app.error = Some(e);
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
            (_, KeyCode::Char(c)) => {
                if c.is_ascii_digit() {
                    let digit = c.to_digit(10).unwrap() as usize;

                    match precommand {
                        Some(precommand) => match precommand {
                            Precommand::Repeat(repeat) => *repeat = *repeat * 10 + digit,
                            _ => *precommand = Precommand::Repeat(digit),
                        },
                        None => *precommand = Some(Precommand::Repeat(digit)),
                    }
                }
            }
            _ => {}
        }
    }
}
