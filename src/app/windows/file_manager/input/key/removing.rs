use std::sync::mpsc::Sender;

use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    app::{AppEvent, InputMode, windows::FileManagerWindow},
    utils::file::delete_path,
};

pub fn handle(
    window: &mut FileManagerWindow,
    input_mode: &InputMode,
    key: &KeyEvent,
    event_tx: &Sender<AppEvent>,
) -> bool {
    if let InputMode::Removing {
        removing_selected,
        path,
    } = input_mode
    {
        match (key.modifiers, key.code) {
            (_, KeyCode::Enter) => {
                if *removing_selected {
                    match delete_path(window.entries.get(window.selected_index), path, event_tx) {
                        Ok(_) => {
                            let _ = event_tx.send(AppEvent::UpdateInputMode(InputMode::Normal {
                                precommand: None,
                            }));
                            let _ = event_tx.send(AppEvent::SetError(None));
                        }
                        Err(e) => {
                            let _ = event_tx.send(AppEvent::SetError(Some(e)));
                            return true;
                        }
                    }
                }

                let _ = event_tx.send(AppEvent::UpdateInputMode(InputMode::Normal {
                    precommand: None,
                }));
                let _ = event_tx.send(AppEvent::SetError(None));
            }
            (_, KeyCode::Char('y') | KeyCode::Char('Y')) => {
                match delete_path(window.entries.get(window.selected_index), path, event_tx) {
                    Ok(_) => {
                        let _ = event_tx.send(AppEvent::UpdateInputMode(InputMode::Normal {
                            precommand: None,
                        }));
                        let _ = event_tx.send(AppEvent::SetError(None));
                    }
                    Err(e) => {
                        let _ = event_tx.send(AppEvent::SetError(Some(e)));
                        return true;
                    }
                }

                let _ = event_tx.send(AppEvent::UpdateInputMode(InputMode::Normal {
                    precommand: None,
                }));
                let _ = event_tx.send(AppEvent::SetError(None));
            }
            _ => {
                return false;
            }
        }

        return true;
    }

    false
}
