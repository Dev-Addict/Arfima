mod directory_entry_builder;
mod directory_entry_type;
mod error;
mod read_directory;

use std::{path::PathBuf, time::SystemTime};

pub use directory_entry_builder::DirectoryEntryBuilder;
pub use directory_entry_type::DirectoryEntryType;
pub use error::Error;
pub use read_directory::read_directory;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct DirectoryEntry {
    name: String,
    path: PathBuf,
    modified: Option<SystemTime>,
    entry_type: DirectoryEntryType,
}

impl DirectoryEntry {
    pub fn builder() -> DirectoryEntryBuilder {
        DirectoryEntryBuilder::new()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn set_path(&mut self, path: PathBuf) {
        self.path = path;
    }

    pub fn modified(&self) -> Option<&SystemTime> {
        self.modified.as_ref()
    }

    pub fn set_modified(&mut self, modified: Option<SystemTime>) {
        self.modified = modified;
    }

    pub fn entry_type(&self) -> &DirectoryEntryType {
        &self.entry_type
    }

    pub fn set_entry_type(&mut self, entry_type: DirectoryEntryType) {
        self.entry_type = entry_type;
    }
}
