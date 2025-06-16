mod key;

use std::sync::mpsc::Sender;

use crossterm::event::Event;
use key::handle_key_event;

use crate::app::{AppEvent, InputMode};

use super::FileManagerWindow;

pub fn handle_event(
    window: &mut FileManagerWindow,
    input_mode: &InputMode,
    event: &Event,
    event_tx: &Sender<AppEvent>,
) {
    if let Event::Key(key) = event {
        handle_key_event(window, input_mode, key, event_tx);
    }
}
