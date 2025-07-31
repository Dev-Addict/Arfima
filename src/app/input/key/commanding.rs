use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{App, InputMode};

// TODO: Implement Enter Key to execute the command

pub fn handle(app: &mut App, key: &KeyEvent) -> bool {
    if let InputMode::Commanding { state } = &mut app.input_mode {
        match (key.modifiers, key.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => app.quit(),
            (_, KeyCode::Esc) => {
                app.input_mode = InputMode::Normal { precommand: None };
                app.error = None;
            }
            (_, KeyCode::Char(c)) => state.insert_char(c),
            (_, KeyCode::Backspace) => state.remove_char(),
            (_, KeyCode::Left) => state.left(),
            (_, KeyCode::Right) => state.right(),
            (_, KeyCode::Home) => state.set_cursor_position(0),
            (_, KeyCode::End) => state.set_cursor_position(state.buffer().len()),
            _ => {
                return false;
            }
        }

        return true;
    }

    false
}
