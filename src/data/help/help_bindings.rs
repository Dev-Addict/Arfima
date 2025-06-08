use std::sync::LazyLock;

use crossterm::event::{KeyCode, KeyModifiers};

use super::KeyBinding;

pub static HELP_BINDINGS: LazyLock<[KeyBinding; 6]> = LazyLock::new(|| {
    [
        KeyBinding::new(
            "exit help",
            &[
                (KeyModifiers::NONE, KeyCode::Esc),
                (KeyModifiers::NONE, KeyCode::Char('q')),
            ],
            "Exit help modal",
        ),
        KeyBinding::new(
            "scroll down",
            &[
                (KeyModifiers::NONE, KeyCode::Down),
                (KeyModifiers::NONE, KeyCode::Char('j')),
            ],
            "Scroll help down",
        ),
        KeyBinding::new(
            "scroll up",
            &[
                (KeyModifiers::NONE, KeyCode::Up),
                (KeyModifiers::NONE, KeyCode::Char('k')),
            ],
            "Scroll help up",
        ),
        KeyBinding::new(
            "quit",
            &[
                (KeyModifiers::CONTROL, KeyCode::Char('c')),
                (KeyModifiers::CONTROL, KeyCode::Char('C')),
            ],
            "Quit the application",
        ),
        KeyBinding::new(
            "jump to top",
            &[
                (KeyModifiers::NONE, KeyCode::Home),
                (KeyModifiers::NONE, KeyCode::Char('g')),
            ],
            "Select first help item",
        ),
        KeyBinding::new(
            "jump to bottom",
            &[
                (KeyModifiers::NONE, KeyCode::End),
                (KeyModifiers::NONE, KeyCode::Char('G')),
            ],
            "Select last help item",
        ),
    ]
});
