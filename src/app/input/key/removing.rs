use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{App, InputMode};

pub fn handle(app: &mut App, key: &KeyEvent) -> bool {
    if let InputMode::Removing {
        removing_selected, ..
    } = &mut app.input_mode
    {
        match (key.modifiers, key.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => app.quit(),
            (_, KeyCode::Esc | KeyCode::Char('n') | KeyCode::Char('N')) => {
                *removing_selected = false;
                app.input_mode = InputMode::Normal { precommand: None };
                app.error = None;
            }
            (_, KeyCode::Char('l') | KeyCode::Right) => {
                *removing_selected = false;
            }
            (_, KeyCode::Char('h') | KeyCode::Left) => {
                *removing_selected = true;
            }
            _ => {
                return false;
            }
        }

        return true;
    }

    false
}
