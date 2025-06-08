use std::sync::LazyLock;

use crossterm::event::{KeyCode, KeyModifiers};

use super::KeyBinding;

pub static ADDING_BINDINGS: LazyLock<[KeyBinding; 3]> = LazyLock::new(|| {
    [
        KeyBinding::new(
            "confirm",
            &[(KeyModifiers::NONE, KeyCode::Enter)],
            "Add new file or directory",
        ),
        KeyBinding::new(
            "cancel",
            &[(KeyModifiers::NONE, KeyCode::Esc)],
            "Cancel adding",
        ),
        KeyBinding::new(
            "quit",
            &[
                (KeyModifiers::CONTROL, KeyCode::Char('c')),
                (KeyModifiers::CONTROL, KeyCode::Char('C')),
            ],
            "Quit the application",
        ),
    ]
});
