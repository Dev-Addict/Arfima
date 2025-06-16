mod key;

use crossterm::event::Event;
use key::handle_key_event;

use super::App;

pub fn handle_event(app: &mut App, event: &Event) {
    if let Event::Key(key) = event {
        handle_key_event(app, key);
    }
}
