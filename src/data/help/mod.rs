mod adding_bindings;
mod help_bindings;
mod normal_bindings;
mod removing_bindings;
mod renaming_bindings;
mod types;

use std::sync::LazyLock;

pub use types::{KeyBinding, ModeKeyBindings};

use crate::app::InputMode;

static HELP: LazyLock<[ModeKeyBindings; 5]> = LazyLock::new(|| {
    [
        ModeKeyBindings::new(
            InputMode::default(),
            "Normal mode: navigate and interact",
            &*normal_bindings::NORMAL_BINDINGS,
        ),
        ModeKeyBindings::new(
            InputMode::adding_default(),
            "Adding mode: type a new name",
            &*adding_bindings::ADDING_BINDINGS,
        ),
        ModeKeyBindings::new(
            InputMode::renaming_default(),
            "Renaming mode: edit the file name",
            &*renaming_bindings::RENAMING_BINDINGS,
        ),
        ModeKeyBindings::new(
            InputMode::removing_default(),
            "Removing mode: confirm deletion",
            &*removing_bindings::REMOVING_BINDINGS,
        ),
        ModeKeyBindings::new(
            InputMode::help_default(),
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
