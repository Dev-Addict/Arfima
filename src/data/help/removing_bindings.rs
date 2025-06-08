use std::sync::LazyLock;

use crossterm::event::{KeyCode, KeyModifiers};

use super::KeyBinding;

pub static REMOVING_BINDINGS: LazyLock<[KeyBinding; 6]> = LazyLock::new(|| {
    [
        KeyBinding::new(
            "confirm",
            &[(KeyModifiers::NONE, KeyCode::Enter)],
            "Confirm the current selection",
        ),
        KeyBinding::new(
            "delete",
            &[
                (KeyModifiers::NONE, KeyCode::Char('y')),
                (KeyModifiers::NONE, KeyCode::Char('Y')),
            ],
            "Confirm and delete",
        ),
        KeyBinding::new(
            "cancel",
            &[
                (KeyModifiers::NONE, KeyCode::Char('n')),
                (KeyModifiers::NONE, KeyCode::Char('N')),
                (KeyModifiers::NONE, KeyCode::Esc),
            ],
            "Cancel deletion",
        ),
        KeyBinding::new(
            "select yes",
            &[
                (KeyModifiers::NONE, KeyCode::Left),
                (KeyModifiers::NONE, KeyCode::Char('h')),
            ],
            "Change selection to yes",
        ),
        KeyBinding::new(
            "select no",
            &[
                (KeyModifiers::NONE, KeyCode::Right),
                (KeyModifiers::NONE, KeyCode::Char('l')),
            ],
            "Change selection to no",
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
