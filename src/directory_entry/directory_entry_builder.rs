use std::{path::PathBuf, time::SystemTime};

use crate::types::NaturalString;

use super::{DirectoryEntry, DirectoryEntryType, Error, Result};

pub struct DirectoryEntryBuilder {
    name: Option<NaturalString>,
    path: Option<PathBuf>,
    modified: Option<SystemTime>,
    entry_type: Option<DirectoryEntryType>,
}

impl DirectoryEntryBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            path: None,
            modified: None,
            entry_type: None,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        let string_name: String = name.into();
        self.name = Some(string_name.into());
        self
    }

    pub fn path(mut self, path: impl Into<PathBuf>) -> Self {
        self.path = Some(path.into());
        self
    }

    pub fn modified(mut self, modified: Option<SystemTime>) -> Self {
        self.modified = modified;
        self
    }

    pub fn entry_type(mut self, entry_type: DirectoryEntryType) -> Self {
        self.entry_type = Some(entry_type);
        self
    }

    pub fn build(self) -> Result<DirectoryEntry> {
        Ok(DirectoryEntry {
            name: self.name.ok_or(Error::MissingName)?,
            path: self.path.ok_or(Error::MissingPath)?,
            modified: self.modified,
            entry_type: self.entry_type.ok_or(Error::MissingEntryType)?,
        })
    }
}
