use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{App, InputMode};

pub fn handle(app: &mut App, key: KeyEvent) {
    match (key.modifiers, key.code) {
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
    }
}
