use crossterm::event::{KeyCode, KeyModifiers};

pub struct KeyBinding<'a> {
    name: &'a str,
    keys: &'a [(KeyModifiers, KeyCode)],
    description: &'a str,
    count: bool,
}

impl<'a> KeyBinding<'a> {
    pub fn new(name: &'a str, keys: &'a [(KeyModifiers, KeyCode)], description: &'a str) -> Self {
        Self {
            name,
            keys,
            description,
            count: false,
        }
    }

    pub fn with_count(
        name: &'a str,
        keys: &'a [(KeyModifiers, KeyCode)],
        description: &'a str,
    ) -> Self {
        Self {
            name,
            keys,
            description,
            count: true,
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

    pub fn count(&self) -> bool {
        self.count
    }
}
