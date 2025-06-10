mod adding;
mod help;
mod normal;
mod removing;
mod renaming;

use crossterm::event::KeyEvent;

use crate::app::{App, InputMode};

pub fn handle_key_event(app: &mut App, key: KeyEvent) {
    match app.input_mode {
        InputMode::Normal { .. } => normal::handle(app, key),
        InputMode::Adding { .. } => adding::handle(app, key),
        InputMode::Renaming { .. } => renaming::handle(app, key),
        InputMode::Removing { .. } => removing::handle(app, key),
        InputMode::Help { .. } => help::handle(app, key),
    }
}
