use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    app::{App, InputMode},
    data::help::get_help_entries_len,
};

pub fn handle(app: &mut App, key: KeyEvent) {
    if let InputMode::Help { selected_index } = &mut app.input_mode {
        match (key.modifiers, key.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => app.quit(),
            (_, KeyCode::Esc | KeyCode::Char('q')) => {
                app.input_mode = InputMode::Normal { precommand: None };
            }
            (_, KeyCode::Down | KeyCode::Char('j')) => {
                if *selected_index + 1 < get_help_entries_len() {
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
                *selected_index = get_help_entries_len().saturating_sub(1)
            }
            _ => {}
        }
    }
}
