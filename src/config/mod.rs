#[derive(Default)]
pub struct NumberConfig {
    active: bool,
    relative: bool,
}

impl NumberConfig {
    pub fn active(&self) -> bool {
        self.active
    }

    pub fn relative(&self) -> bool {
        self.relative
    }
}

#[derive(Default)]
pub struct Config {
    number: NumberConfig,
}

impl Config {
    pub fn number(&self) -> &NumberConfig {
        &self.number
    }
}
