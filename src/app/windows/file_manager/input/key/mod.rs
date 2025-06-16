mod adding;
mod normal;
mod removing;
mod renaming;

use std::sync::mpsc::Sender;

use crossterm::event::{KeyEvent, KeyEventKind};

use crate::app::{AppEvent, InputMode};

use super::FileManagerWindow;

pub fn handle_key_event(
    window: &mut FileManagerWindow,
    input_mode: &InputMode,
    key: &KeyEvent,
    event_tx: &Sender<AppEvent>,
) {
    if key.kind != KeyEventKind::Press {
        return;
    }

    normal::handle(window, input_mode, key, event_tx);
    adding::handle(window, input_mode, key, event_tx);
    renaming::handle(window, input_mode, key, event_tx);
    removing::handle(window, input_mode, key, event_tx);
}
