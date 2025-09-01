mod common_entries;
mod error;
mod history;
mod number;

use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use common_entries::CommonEntriesConfig;
pub use error::Error;
use history::HistoryConfig;
use number::NumberConfig;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Default, Deserialize, Serialize)]
pub struct Config {
    #[serde(default, skip_serializing)]
    path: PathBuf,
    #[serde(default)]
    number: NumberConfig,
    #[serde(default)]
    common_entries: CommonEntriesConfig,
    #[serde(default)]
    history: HistoryConfig,
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

    pub fn history(&self) -> &HistoryConfig {
        &self.history
    }

    pub fn mut_history(&mut self) -> &mut HistoryConfig {
        &mut self.history
    }

    pub fn save(&self) -> Result<()> {
        let contents: String = self.try_into()?;
        fs::write(self.path.as_path(), contents.as_bytes())?;

        Ok(())
    }
}

impl TryFrom<PathBuf> for Config {
    type Error = Error;

    fn try_from(value: PathBuf) -> std::result::Result<Self, Self::Error> {
        let contents = fs::read_to_string(value.clone())?;

        let mut config = toml::from_str::<Config>(&contents)?;

        config.path = value;

        Ok(config)
    }
}

impl TryInto<String> for &Config {
    type Error = Error;

    fn try_into(self) -> std::result::Result<String, Self::Error> {
        Ok(toml::to_string(self)?)
    }
}
