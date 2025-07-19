use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    app::{App, InputMode},
    utils::file::open_file_with_app,
};

pub fn handle(app: &mut App, key: &KeyEvent) -> bool {
    if let InputMode::Opening {
        path,
        apps,
        selected_index,
    } = &mut app.input_mode
    {
        match (key.modifiers, key.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => app.quit(),
            (_, KeyCode::Esc) => {
                app.input_mode = InputMode::Normal { precommand: None };
                app.error = None;
            }
            (_, KeyCode::Char('j') | KeyCode::Down) => {
                *selected_index = selected_index
                    .saturating_add(1)
                    .min(apps.len().saturating_sub(1));
            }
            (_, KeyCode::Char('k') | KeyCode::Up) => {
                *selected_index = selected_index.saturating_sub(1);
            }
            (_, KeyCode::Enter) => {
                if let Some(selected_app) = apps.get(*selected_index) {
                    match open_file_with_app(selected_app.as_str(), path.as_str()) {
                        Ok(_) => {
                            app.input_mode = InputMode::Normal { precommand: None };
                            app.error = None;
                        }
                        Err(e) => {
                            app.error = Some(e.into());
                        }
                    }
                }
            }
            (_, KeyCode::Home) => *selected_index = 0,
            (_, KeyCode::End) => *selected_index = apps.len().saturating_sub(1),
            _ => {
                return false;
            }
        }

        return true;
    }

    false
}
