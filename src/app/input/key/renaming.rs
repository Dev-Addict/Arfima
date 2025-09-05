use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{App, InputMode};

pub fn handle(app: &mut App, key: &KeyEvent) -> bool {
    if let InputMode::Renaming { state, .. } = &mut app.input_mode {
        match (key.modifiers, key.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => app.quit(),
            (_, KeyCode::Esc) => {
                app.input_mode = InputMode::Normal { precommand: None };
                app.error = None;
            }
            (_, KeyCode::Char(c)) => state.insert_char(c),
            (_, KeyCode::Backspace) => state.remove_char(),
            (KeyModifiers::NONE, KeyCode::Left) => state.left(false),
            (KeyModifiers::SHIFT, KeyCode::Left) => state.left(true),
            (KeyModifiers::NONE, KeyCode::Right) => state.right(false),
            (KeyModifiers::SHIFT, KeyCode::Right) => state.right(true),
            (_, KeyCode::Home) => state.set_cursor_position(0),
            (_, KeyCode::End) => state.set_cursor_position(state.buffer().chars().count()),
            _ => {
                return false;
            }
        }

        return true;
    }

    false
}
