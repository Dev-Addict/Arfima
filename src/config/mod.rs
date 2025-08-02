mod error;

use std::{fs, path::PathBuf};

use serde::Deserialize;

use error::Error;

#[derive(Default, Deserialize)]
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

#[derive(Default, Deserialize)]
pub struct Config {
    number: NumberConfig,
}

impl Config {
    pub fn number(&self) -> &NumberConfig {
        &self.number
    }
}

impl TryFrom<PathBuf> for Config {
    type Error = Error;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        let contents = fs::read_to_string(value)?;

        Ok(toml::from_str::<Config>(&contents)?)
    }
}
