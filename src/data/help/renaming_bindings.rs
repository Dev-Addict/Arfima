use std::sync::LazyLock;

use crossterm::event::{KeyCode, KeyModifiers};

use super::KeyBinding;

pub static RENAMING_BINDINGS: LazyLock<[KeyBinding; 3]> = LazyLock::new(|| {
    [
        KeyBinding::new(
            "confirm",
            &[(KeyModifiers::NONE, KeyCode::Enter)],
            "Rename selected entry",
        ),
        KeyBinding::new(
            "cancel",
            &[(KeyModifiers::NONE, KeyCode::Esc)],
            "Cancel renaming",
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
