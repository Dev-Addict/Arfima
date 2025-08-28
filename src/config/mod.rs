mod error;

use std::{
    env, fs,
    path::{Path, PathBuf},
};

use directories::BaseDirs;
use serde::Deserialize;

use error::Error;

fn expand_tilde(path: &str) -> PathBuf {
    if let Some(stripped) = path.strip_prefix("~/") {
        if let Some(base_dirs) = BaseDirs::new() {
            return base_dirs.home_dir().join(stripped);
        }

        if let Some(home) = env::var_os("HOME").or_else(|| env::var_os("USERPROFILE")) {
            return Path::new(&home).join(stripped);
        }
    }

    PathBuf::from(path)
}

fn deserialize_paths<'de, D>(deserializer: D) -> Result<Vec<PathBuf>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let raw: Vec<String> = Vec::deserialize(deserializer)?;
    Ok(raw.into_iter().map(|s| expand_tilde(&s)).collect())
}

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

#[derive(Deserialize)]
pub struct CommonEntriesConfig {
    #[serde(default)]
    user_dirs: bool,
    #[serde(default, deserialize_with = "deserialize_paths")]
    other_paths: Vec<PathBuf>,
}

impl CommonEntriesConfig {
    pub fn user_dirs(&self) -> bool {
        self.user_dirs
    }

    pub fn set_user_dirs(&mut self, user_dirs: bool) {
        self.user_dirs = user_dirs;
    }

    pub fn other_paths(&self) -> &Vec<PathBuf> {
        &self.other_paths
    }

    pub fn set_other_paths<T: Into<Vec<PathBuf>>>(&mut self, other_paths: T) {
        self.other_paths = other_paths.into();
    }

    pub fn mut_other_paths(&mut self) -> &mut Vec<PathBuf> {
        &mut self.other_paths
    }
}

impl Default for CommonEntriesConfig {
    fn default() -> Self {
        Self {
            user_dirs: true,
            other_paths: Vec::default(),
        }
    }
}

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
