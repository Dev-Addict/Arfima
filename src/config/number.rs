use serde::Deserialize;

#[derive(Default, Deserialize)]
pub struct NumberConfig {
    #[serde(default)]
    active: bool,
    #[serde(default)]
    relative: bool,
}

impl NumberConfig {
    pub fn active(&self) -> bool {
        self.active
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn relative(&self) -> bool {
        self.relative
    }

    pub fn set_relative(&mut self, relative: bool) {
        self.relative = relative;
    }
}
