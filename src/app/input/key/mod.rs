mod adding;
mod commanding;
mod help;
mod normal;
mod opening;
mod removing;
mod renaming;

use crossterm::event::{KeyEvent, KeyEventKind};

use crate::app::App;

pub fn handle_key_event(app: &mut App, key: &KeyEvent) -> bool {
    if key.kind != KeyEventKind::Press {
        return false;
    }

    if normal::handle(app, key) {
        return true;
    }
    if adding::handle(app, key) {
        return true;
    }
    if renaming::handle(app, key) {
        return true;
    }
    if removing::handle(app, key) {
        return true;
    }
    if opening::handle(app, key) {
        return true;
    }
    if commanding::handle(app, key) {
        return true;
    }
    help::handle(app, key)
}
