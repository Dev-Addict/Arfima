use crossterm::event::{KeyCode, KeyModifiers};

use crate::app::Precommand;

pub struct KeyBinding<'a> {
    name: &'a str,
    keys: &'a [(KeyModifiers, KeyCode)],
    description: &'a str,
    precommand: Option<Precommand>,
}

impl<'a> KeyBinding<'a> {
    pub fn new(name: &'a str, keys: &'a [(KeyModifiers, KeyCode)], description: &'a str) -> Self {
        Self {
            name,
            keys,
            description,
            precommand: None,
        }
    }

    pub fn with_precommand(
        name: &'a str,
        keys: &'a [(KeyModifiers, KeyCode)],
        description: &'a str,
        precommand: Precommand,
    ) -> Self {
        Self {
            name,
            keys,
            description,
            precommand: Some(precommand),
        }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn keys(&self) -> &[(KeyModifiers, KeyCode)] {
        self.keys
    }

    pub fn description(&self) -> &str {
        self.description
    }

    pub fn precommand(&self) -> Option<&Precommand> {
        self.precommand.as_ref()
    }
}
