mod adding_bindings;
mod help_bindings;
mod normal_bindings;
mod removing_bindings;
mod renaming_bindings;
mod types;

use std::sync::LazyLock;

pub use types::{KeyBinding, ModeKeyBindings};

use crate::app::{InputMode, InputState};

static HELP: LazyLock<[ModeKeyBindings; 5]> = LazyLock::new(|| {
    [
        ModeKeyBindings::new(
            InputMode::Normal,
            "Normal mode: navigate and interact",
            &*normal_bindings::NORMAL_BINDINGS,
        ),
        ModeKeyBindings::new(
            InputMode::Adding {
                state: InputState::new("", 0),
            },
            "Adding mode: type a new name",
            &*adding_bindings::ADDING_BINDINGS,
        ),
        ModeKeyBindings::new(
            InputMode::Renaming {
                original: "".into(),
                state: InputState::new("", 0),
            },
            "Renaming mode: edit the file name",
            &*renaming_bindings::RENAMING_BINDINGS,
        ),
        ModeKeyBindings::new(
            InputMode::Removing { path: "".into() },
            "Removing mode: confirm deletion",
            &*removing_bindings::REMOVING_BINDINGS,
        ),
        ModeKeyBindings::new(
            InputMode::Help { selected_index: 0 },
            "Help mode: read keybindings",
            &*help_bindings::HELP_BINDINGS,
        ),
    ]
});

static HELP_ENTRIES_LEN: LazyLock<usize> =
    LazyLock::new(|| HELP.iter().map(|mode| mode.items().len()).sum::<usize>() + HELP.len());

pub fn get_help() -> &'static [ModeKeyBindings<'static>] {
    &*HELP
}

pub fn get_help_entries_len() -> usize {
    *HELP_ENTRIES_LEN
}
