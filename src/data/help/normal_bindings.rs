use std::sync::LazyLock;

use crossterm::event::{KeyCode, KeyModifiers};

use super::KeyBinding;

pub static NORMAL_BINDINGS: LazyLock<[KeyBinding; 12]> = LazyLock::new(|| {
    [
        KeyBinding::new(
            "quit",
            &[
                (KeyModifiers::NONE, KeyCode::Char('q')),
                (KeyModifiers::CONTROL, KeyCode::Char('c')),
            ],
            "Quit the application",
        ),
        KeyBinding::new(
            "reset",
            &[(KeyModifiers::NONE, KeyCode::Esc)],
            "Reset the normal mode",
        ),
        KeyBinding::new(
            "navigate down",
            &[
                (KeyModifiers::NONE, KeyCode::Down),
                (KeyModifiers::NONE, KeyCode::Char('j')),
            ],
            "Move selection down",
        ),
        KeyBinding::new(
            "navigate up",
            &[
                (KeyModifiers::NONE, KeyCode::Up),
                (KeyModifiers::NONE, KeyCode::Char('k')),
            ],
            "Move selection up",
        ),
        KeyBinding::new(
            "enter help mode",
            &[(KeyModifiers::CONTROL, KeyCode::Char('h'))],
            "Show help modal",
        ),
        KeyBinding::new(
            "Go up",
            &[
                (KeyModifiers::NONE, KeyCode::Left),
                (KeyModifiers::NONE, KeyCode::Char('h')),
                (KeyModifiers::NONE, KeyCode::Backspace),
            ],
            "Navigate to parent directory",
        ),
        KeyBinding::new(
            "open",
            &[
                (KeyModifiers::NONE, KeyCode::Right),
                (KeyModifiers::NONE, KeyCode::Char('l')),
                (KeyModifiers::NONE, KeyCode::Enter),
            ],
            "Open selected file or directory",
        ),
        KeyBinding::new(
            "add entry",
            &[(KeyModifiers::NONE, KeyCode::Char('a'))],
            "Create a new file or directory changing the mode to ADDING",
        ),
        KeyBinding::new(
            "rename entry",
            &[(KeyModifiers::NONE, KeyCode::Char('r'))],
            "Rename selected entry by change mode to RENAMING",
        ),
        KeyBinding::new(
            "remove entry",
            &[(KeyModifiers::NONE, KeyCode::Char('d'))],
            "Delete selected entry by change mode to REMOVING",
        ),
        KeyBinding::new(
            "jump to top",
            &[
                (KeyModifiers::NONE, KeyCode::Home),
                (KeyModifiers::NONE, KeyCode::Char('g')),
            ],
            "Select first entry",
        ),
        KeyBinding::new(
            "jump to bottom",
            &[
                (KeyModifiers::NONE, KeyCode::End),
                (KeyModifiers::NONE, KeyCode::Char('G')),
            ],
            "Select last entry",
        ),
    ]
});
