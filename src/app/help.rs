use std::sync::LazyLock;

use crossterm::event::{KeyCode, KeyModifiers};

use super::{InputMode, InputState};

pub struct KeyBinding<'a> {
    pub name: &'a str,
    pub keys: &'a [(KeyModifiers, KeyCode)],
    pub description: &'a str,
}

pub struct ModeKeybindings<'a> {
    pub mode: InputMode,
    pub description: &'a str,
    pub items: &'a [KeyBinding<'a>],
}

static HELP: LazyLock<[ModeKeybindings; 5]> = LazyLock::new(|| {
    let normal_bindings: &'static [KeyBinding<'static>] = Box::leak(Box::new([
        KeyBinding {
            name: "quit",
            keys: &[
                (KeyModifiers::NONE, KeyCode::Char('q')),
                (KeyModifiers::NONE, KeyCode::Esc),
                (KeyModifiers::CONTROL, KeyCode::Char('c')),
            ],
            description: "Quit the application",
        },
        KeyBinding {
            name: "navigate down",
            keys: &[
                (KeyModifiers::NONE, KeyCode::Down),
                (KeyModifiers::NONE, KeyCode::Char('j')),
            ],
            description: "Move selection down",
        },
        KeyBinding {
            name: "navigate up",
            keys: &[
                (KeyModifiers::NONE, KeyCode::Up),
                (KeyModifiers::NONE, KeyCode::Char('k')),
            ],
            description: "Move selection up",
        },
        KeyBinding {
            name: "enter help mode",
            keys: &[(KeyModifiers::CONTROL, KeyCode::Char('h'))],
            description: "Show help modal",
        },
        KeyBinding {
            name: "Go up",
            keys: &[
                (KeyModifiers::NONE, KeyCode::Left),
                (KeyModifiers::NONE, KeyCode::Char('h')),
                (KeyModifiers::NONE, KeyCode::Backspace),
            ],
            description: "Navigate to parent directory",
        },
        KeyBinding {
            name: "open",
            keys: &[
                (KeyModifiers::NONE, KeyCode::Right),
                (KeyModifiers::NONE, KeyCode::Char('l')),
                (KeyModifiers::NONE, KeyCode::Enter),
            ],
            description: "Open selected file or directory",
        },
        KeyBinding {
            name: "add entry",
            keys: &[(KeyModifiers::NONE, KeyCode::Char('a'))],
            description: "Create a new file or directory by changing the mode to ADDING",
        },
        KeyBinding {
            name: "rename entry",
            keys: &[(KeyModifiers::NONE, KeyCode::Char('r'))],
            description: "Rename selected entry by changing the mode to RENAMING",
        },
        KeyBinding {
            name: "remove entry",
            keys: &[(KeyModifiers::NONE, KeyCode::Char('d'))],
            description: "Delete selected entry by changing the mode to REMOVING",
        },
        KeyBinding {
            name: "jump to top",
            keys: &[
                (KeyModifiers::NONE, KeyCode::Home),
                (KeyModifiers::NONE, KeyCode::Char('g')),
            ],
            description: "Select first entry",
        },
        KeyBinding {
            name: "jump to bottom",
            keys: &[
                (KeyModifiers::NONE, KeyCode::End),
                (KeyModifiers::NONE, KeyCode::Char('G')),
            ],
            description: "Select last entry",
        },
    ]));
    let adding_bindings: &'static [KeyBinding<'static>] = Box::leak(Box::new([
        KeyBinding {
            name: "confirm",
            keys: &[(KeyModifiers::NONE, KeyCode::Enter)],
            description: "Add new file or directory",
        },
        KeyBinding {
            name: "cancel",
            keys: &[(KeyModifiers::NONE, KeyCode::Esc)],
            description: "Cancel adding",
        },
        KeyBinding {
            name: "quit",
            keys: &[
                (KeyModifiers::CONTROL, KeyCode::Char('c')),
                (KeyModifiers::CONTROL, KeyCode::Char('C')),
            ],
            description: "Quit the application",
        },
    ]));
    let renaming_bindings: &'static [KeyBinding<'static>] = Box::leak(Box::new([
        KeyBinding {
            name: "confirm",
            keys: &[(KeyModifiers::NONE, KeyCode::Enter)],
            description: "Rename selected entry",
        },
        KeyBinding {
            name: "cancel",
            keys: &[(KeyModifiers::NONE, KeyCode::Esc)],
            description: "Cancel renaming",
        },
        KeyBinding {
            name: "quit",
            keys: &[
                (KeyModifiers::CONTROL, KeyCode::Char('c')),
                (KeyModifiers::CONTROL, KeyCode::Char('C')),
            ],
            description: "Quit the application",
        },
    ]));
    let removing_bindings: &'static [KeyBinding<'static>] = Box::leak(Box::new([
        KeyBinding {
            name: "confirm",
            keys: &[(KeyModifiers::NONE, KeyCode::Enter)],
            description: "Confirm the current selection",
        },
        KeyBinding {
            name: "delete",
            keys: &[
                (KeyModifiers::NONE, KeyCode::Char('y')),
                (KeyModifiers::NONE, KeyCode::Char('Y')),
            ],
            description: "Confirm and delete",
        },
        KeyBinding {
            name: "cancel",
            keys: &[
                (KeyModifiers::NONE, KeyCode::Char('n')),
                (KeyModifiers::NONE, KeyCode::Char('N')),
                (KeyModifiers::NONE, KeyCode::Esc),
            ],
            description: "Cancel deletion",
        },
        KeyBinding {
            name: "select yes",
            keys: &[
                (KeyModifiers::NONE, KeyCode::Left),
                (KeyModifiers::NONE, KeyCode::Char('h')),
            ],
            description: "Change selection to yes",
        },
        KeyBinding {
            name: "select no",
            keys: &[
                (KeyModifiers::NONE, KeyCode::Right),
                (KeyModifiers::NONE, KeyCode::Char('l')),
            ],
            description: "Change selection to no",
        },
        KeyBinding {
            name: "quit",
            keys: &[
                (KeyModifiers::CONTROL, KeyCode::Char('c')),
                (KeyModifiers::CONTROL, KeyCode::Char('C')),
            ],
            description: "Quit the application",
        },
    ]));
    let help_bindings: &'static [KeyBinding<'static>] = Box::leak(Box::new([
        KeyBinding {
            name: "exit help",
            keys: &[
                (KeyModifiers::NONE, KeyCode::Esc),
                (KeyModifiers::NONE, KeyCode::Char('q')),
            ],
            description: "Exit help modal",
        },
        KeyBinding {
            name: "scroll down",
            keys: &[
                (KeyModifiers::NONE, KeyCode::Down),
                (KeyModifiers::NONE, KeyCode::Char('j')),
            ],
            description: "Scroll help down",
        },
        KeyBinding {
            name: "scroll up",
            keys: &[
                (KeyModifiers::NONE, KeyCode::Up),
                (KeyModifiers::NONE, KeyCode::Char('k')),
            ],
            description: "Scroll help up",
        },
        KeyBinding {
            name: "quit",
            keys: &[
                (KeyModifiers::CONTROL, KeyCode::Char('c')),
                (KeyModifiers::CONTROL, KeyCode::Char('C')),
            ],
            description: "Quit the application",
        },
        KeyBinding {
            name: "jump to top",
            keys: &[
                (KeyModifiers::NONE, KeyCode::Home),
                (KeyModifiers::NONE, KeyCode::Char('g')),
            ],
            description: "Select first help item",
        },
        KeyBinding {
            name: "jump to bottom",
            keys: &[
                (KeyModifiers::NONE, KeyCode::End),
                (KeyModifiers::NONE, KeyCode::Char('G')),
            ],
            description: "Select last help item",
        },
    ]));

    [
        ModeKeybindings {
            mode: InputMode::Normal,
            description: "Normal mode: navigate and interact",
            items: normal_bindings,
        },
        ModeKeybindings {
            mode: InputMode::Adding {
                state: InputState {
                    buffer: "".into(),
                    cursor_position: 0,
                },
            },
            description: "Adding mode: type a new name",
            items: adding_bindings,
        },
        ModeKeybindings {
            mode: InputMode::Renaming {
                original: "".into(),
                state: InputState {
                    buffer: "".into(),
                    cursor_position: 0,
                },
            },
            description: "Renaming mode: edit the file name",
            items: renaming_bindings,
        },
        ModeKeybindings {
            mode: InputMode::Removing { path: "".into() },
            description: "Removing mode: confirm deletion",
            items: removing_bindings,
        },
        ModeKeybindings {
            mode: InputMode::Help { selected_index: 0 },
            description: "Help mode: read keybindings",
            items: help_bindings,
        },
    ]
});

static HELP_ENTERIES_LEN: LazyLock<usize> = LazyLock::new(|| {
    let mut len = HELP.len();

    for mode in HELP.iter() {
        len += mode.items.len();
    }

    len
});

pub fn get_help() -> &'static [ModeKeybindings<'static>] {
    &*HELP
}

pub fn get_help_enteries_len() -> usize {
    *HELP_ENTERIES_LEN
}
