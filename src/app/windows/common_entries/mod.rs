mod input;
mod window_impl;

use std::sync::LazyLock;

use directories::UserDirs;
use ratatui::layout::Direction;

use crate::{
    app::{
        window::{Window, WindowSize, generate_window_id},
        windows::SplitWindow,
    },
    config::Config,
    directory_entry::DirectoryEntry,
};

static COMMON_ENTRIES_WINDOW_ID: LazyLock<u32> = LazyLock::new(generate_window_id);

pub struct CommonEntriesWindow {
    entries: Vec<DirectoryEntry>,
    selected_index: usize,
    window_size: WindowSize,
    is_open: bool,
}

impl CommonEntriesWindow {
    fn entries(config: &Config) -> Vec<DirectoryEntry> {
        let mut entries = config
            .common_entries()
            .other_paths()
            .iter()
            .filter_map(|path| path.try_into().ok())
            .collect::<Vec<DirectoryEntry>>();

        if config.common_entries().user_dirs()
            && let Some(user_dirs) = UserDirs::new()
        {
            let mut user_dirs = vec![
                Some(user_dirs.home_dir()),
                user_dirs.audio_dir(),
                user_dirs.desktop_dir(),
                user_dirs.document_dir(),
                user_dirs.download_dir(),
                user_dirs.font_dir(),
                user_dirs.picture_dir(),
                user_dirs.public_dir(),
                user_dirs.template_dir(),
                user_dirs.video_dir(),
            ]
            .into_iter()
            .filter_map(|path| match path {
                Some(path) => (&path.to_path_buf()).try_into().ok(),
                None => None,
            })
            .collect::<Vec<DirectoryEntry>>();

            user_dirs.append(&mut entries);

            user_dirs
        } else {
            entries
        }
    }

    fn new(config: &Config) -> Self {
        Self {
            entries: Self::entries(config),
            selected_index: 0,
            window_size: WindowSize::DefaultSize(40),
            is_open: false,
        }
    }

    pub fn toggle(window: Box<dyn Window>, config: &Config) -> Option<Box<dyn Window>> {
        if window.includes(*COMMON_ENTRIES_WINDOW_ID) {
            window.remove(*COMMON_ENTRIES_WINDOW_ID)
        } else {
            Some(Box::new(SplitWindow::with_window_size(
                Direction::Horizontal,
                vec![Box::new(Self::new(config)), window],
                WindowSize::Default,
            )))
        }
    }
}
