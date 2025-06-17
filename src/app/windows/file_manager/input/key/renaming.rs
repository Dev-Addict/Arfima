use std::sync::mpsc::Sender;

use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    app::{AppEvent, InputMode, windows::FileManagerWindow},
    utils::file::rename_path,
};

pub fn handle(
    window: &mut FileManagerWindow,
    input_mode: &InputMode,
    key: &KeyEvent,
    event_tx: &Sender<AppEvent>,
) -> bool {
    if let InputMode::Renaming { state, original } = input_mode {
        if let (_, KeyCode::Enter) = (key.modifiers, key.code) {
            match rename_path(&window.directory, state, original, event_tx) {
                Ok(_) => {
                    let _ = event_tx.send(AppEvent::UpdateInputMode(InputMode::Normal {
                        precommand: None,
                    }));
                    let _ = event_tx.send(AppEvent::SetError(None));
                }
                Err(e) => {
                    let _ = event_tx.send(AppEvent::SetError(Some(e)));
                }
            }

            return true;
        }

        return false;
    }

    false
}
