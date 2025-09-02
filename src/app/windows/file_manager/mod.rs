mod input;
mod window_impl;

use std::path::Path;

use crate::{
    app::{
        Error, Result,
        window::{WindowSize, generate_window_id},
    },
    directory_entry::{DirectoryEntry, read_directory},
};

use super::SplitWindow;

#[derive(Clone)]
pub struct FileManagerWindow {
    id: u32,
    directory: String,
    entries: Vec<DirectoryEntry>,
    selected_index: usize,
    window_size: WindowSize,
}

impl FileManagerWindow {
    pub fn new(directory: &str) -> Result<Self> {
        let path = Path::new(directory);

        if !path.is_dir() {
            return Err(Error::InvalidDirectoryPath(directory.into()));
        }

        Ok(Self {
            id: generate_window_id(),
            directory: directory.into(),
            entries: read_directory(path)?,
            selected_index: 0,
            window_size: WindowSize::Default,
        })
    }

    pub fn with_id_and_window_size(
        directory: &str,
        id: u32,
        window_size: WindowSize,
    ) -> Result<Self> {
        let path = Path::new(directory);

        if !path.is_dir() {
            return Err(Error::InvalidDirectoryPath(directory.into()));
        }

        Ok(Self {
            id,
            directory: directory.into(),
            entries: read_directory(path)?,
            selected_index: 0,
            window_size,
        })
    }

    pub fn set_directory(&mut self, directory: String) -> Result<()> {
        let path = Path::new(&directory);

        if !path.is_dir() {
            return Err(Error::InvalidDirectoryPath(directory));
        }

        self.entries = read_directory(path)?;
        self.directory = directory;
        self.selected_index = 0;

        Ok(())
    }
}
