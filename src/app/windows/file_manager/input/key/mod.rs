mod adding;
mod normal;
mod removing;
mod renaming;

use crossbeam::channel::Sender;
use crossterm::event::{KeyEvent, KeyEventKind};

use crate::app::{AppEvent, InputMode};

use super::FileManagerWindow;

pub fn handle_key_event(
    window: &mut FileManagerWindow,
    input_mode: &InputMode,
    key: &KeyEvent,
    event_tx: &Sender<AppEvent>,
) -> bool {
    if key.kind != KeyEventKind::Press {
        return false;
    }

    if normal::handle(window, input_mode, key, event_tx) {
        return true;
    }
    if adding::handle(window, input_mode, key, event_tx) {
        return true;
    }
    if renaming::handle(window, input_mode, key, event_tx) {
        return true;
    }
    removing::handle(window, input_mode, key, event_tx)
}
