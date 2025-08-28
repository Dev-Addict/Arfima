mod key;

use std::sync::mpsc::Sender;

use crossterm::event::Event;
use key::handle_key_event;

use crate::app::{AppEvent, InputMode};

use super::CommonEntriesWindow;

pub fn handle_event(
    window: &mut CommonEntriesWindow,
    input_mode: &InputMode,
    event: &Event,
    event_tx: &Sender<AppEvent>,
) -> bool {
    if let Event::Key(key) = event {
        return handle_key_event(window, input_mode, key, event_tx);
    }

    false
}
