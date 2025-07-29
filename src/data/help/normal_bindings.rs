use std::sync::LazyLock;

use crossterm::event::{KeyCode, KeyModifiers};

use crate::app::Precommand;

use super::KeyBinding;

pub static NORMAL_BINDINGS: LazyLock<[KeyBinding; 20]> = LazyLock::new(|| {
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
        KeyBinding::with_precommand(
            "navigate down",
            &[
                (KeyModifiers::NONE, KeyCode::Down),
                (KeyModifiers::NONE, KeyCode::Char('j')),
            ],
            "Move selection down",
            Precommand::Repeat(0),
        ),
        KeyBinding::with_precommand(
            "navigate up",
            &[
                (KeyModifiers::NONE, KeyCode::Up),
                (KeyModifiers::NONE, KeyCode::Char('k')),
            ],
            "Move selection up",
            Precommand::Repeat(0),
        ),
        KeyBinding::new(
            "enter help mode",
            &[(KeyModifiers::CONTROL, KeyCode::Char('h'))],
            "Show help modal",
        ),
        KeyBinding::with_precommand(
            "Go up",
            &[
                (KeyModifiers::NONE, KeyCode::Left),
                (KeyModifiers::NONE, KeyCode::Char('h')),
                (KeyModifiers::NONE, KeyCode::Backspace),
            ],
            "Navigate to parent directory",
            Precommand::Repeat(0),
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
        KeyBinding::with_precommand(
            "next window",
            &[
                (KeyModifiers::CONTROL, KeyCode::Char('w')),
                (KeyModifiers::NONE, KeyCode::Char('j')),
                (KeyModifiers::NONE, KeyCode::Right),
            ],
            "Iterate through windows to the next window",
            Precommand::RepeatWindow(0),
        ),
        KeyBinding::with_precommand(
            "prev window",
            &[
                (KeyModifiers::NONE, KeyCode::Char('k')),
                (KeyModifiers::NONE, KeyCode::Left),
            ],
            "Iterate through windows to the previous window",
            Precommand::RepeatWindow(0),
        ),
        KeyBinding::with_precommand(
            "split h",
            &[(KeyModifiers::NONE, KeyCode::Char('h'))],
            "Open a new window in a horizontal split",
            Precommand::RepeatWindow(0),
        ),
        KeyBinding::with_precommand(
            "split v",
            &[(KeyModifiers::NONE, KeyCode::Char('v'))],
            "Open a new window in a verticall split",
            Precommand::RepeatWindow(0),
        ),
        KeyBinding::with_precommand(
            "inc win width",
            &[(KeyModifiers::NONE, KeyCode::Char('+'))],
            "Increase focused window size horizontally",
            Precommand::RepeatWindow(0),
        ),
        KeyBinding::with_precommand(
            "dec win width",
            &[(KeyModifiers::NONE, KeyCode::Char('-'))],
            "Decrease focused window size horizontally",
            Precommand::RepeatWindow(0),
        ),
        KeyBinding::with_precommand(
            "inc win height",
            &[(KeyModifiers::NONE, KeyCode::Char('='))],
            "Increase focused window size vertically",
            Precommand::RepeatWindow(0),
        ),
        KeyBinding::with_precommand(
            "dec win height",
            &[(KeyModifiers::NONE, KeyCode::Char('_'))],
            "Decrease focused window size vertically",
            Precommand::RepeatWindow(0),
        ),
    ]
});
