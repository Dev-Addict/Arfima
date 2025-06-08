use crate::app::InputMode;

use super::KeyBinding;

pub struct ModeKeyBindings<'a> {
    mode: InputMode,
    description: &'a str,
    items: &'a [KeyBinding<'a>],
}

impl<'a> ModeKeyBindings<'a> {
    pub fn new(mode: InputMode, description: &'a str, items: &'a [KeyBinding]) -> Self {
        Self {
            mode,
            description,
            items,
        }
    }

    pub fn mode(&self) -> &InputMode {
        &self.mode
    }

    pub fn description(&self) -> &str {
        self.description
    }

    pub fn items(&self) -> &[KeyBinding] {
        self.items
    }
}
