mod adding;
mod help;
mod normal;
mod removing;
mod renaming;

use crossterm::event::{KeyEvent, KeyEventKind};

use crate::app::App;

pub fn handle_key_event(app: &mut App, key: &KeyEvent) {
    if key.kind != KeyEventKind::Press {
        return;
    }

    normal::handle(app, key);
    adding::handle(app, key);
    renaming::handle(app, key);
    removing::handle(app, key);
    help::handle(app, key);
}
