mod normal;

use std::sync::mpsc::Sender;

use crossterm::event::{KeyEvent, KeyEventKind};

use crate::app::{AppEvent, InputMode};

use super::UserDirectoriesWindow;

pub fn handle_key_event(
    window: &mut UserDirectoriesWindow,
    input_mode: &InputMode,
    key: &KeyEvent,
    event_tx: &Sender<AppEvent>,
) -> bool {
    if key.kind != KeyEventKind::Press {
        return false;
    }

    normal::handle(window, input_mode, key, event_tx)
}
