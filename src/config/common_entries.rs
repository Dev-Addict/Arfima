use std::path::PathBuf;

use serde::Deserialize;

use crate::utils::file::expand_tilde;

fn deserialize_paths<'de, D>(deserializer: D) -> Result<Vec<PathBuf>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let raw: Vec<String> = Vec::deserialize(deserializer)?;
    Ok(raw.into_iter().map(|s| expand_tilde(&s)).collect())
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
