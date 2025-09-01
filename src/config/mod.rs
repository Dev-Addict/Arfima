mod common_entries;
mod error;
mod number;

use std::{fs, path::PathBuf};

use serde::Deserialize;

use common_entries::CommonEntriesConfig;
use error::Error;
use number::NumberConfig;

#[derive(Default, Deserialize)]
pub struct Config {
    #[serde(default)]
    number: NumberConfig,
    #[serde(default)]
    common_entries: CommonEntriesConfig,
}

impl Config {
    pub fn number(&self) -> &NumberConfig {
        &self.number
    }

    pub fn mut_number(&mut self) -> &mut NumberConfig {
        &mut self.number
    }

    pub fn common_entries(&self) -> &CommonEntriesConfig {
        &self.common_entries
    }

    pub fn mut_common_entries(&mut self) -> &mut CommonEntriesConfig {
        &mut self.common_entries
    }
}

//TODO: Save Config function and command

impl TryFrom<PathBuf> for Config {
    type Error = Error;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        let contents = fs::read_to_string(value)?;

        Ok(toml::from_str::<Config>(&contents)?)
    }
}
