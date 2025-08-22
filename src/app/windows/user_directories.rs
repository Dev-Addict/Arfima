use std::{
    fs,
    sync::{Arc, Mutex, OnceLock},
};

use directories::UserDirs;

use crate::{
    app::window::{Window, WindowSize},
    directory_entry::{DirectoryEntry, DirectoryEntryType},
};

pub struct UserDirectoriesWindow {
    entries: Vec<DirectoryEntry>,
    selected_index: usize,
    window_size: WindowSize,
    is_open: bool,
}

static USER_DIRECTORIES_WINDOW: OnceLock<Arc<Mutex<UserDirectoriesWindow>>> = OnceLock::new();

impl UserDirectoriesWindow {
    fn new() -> Self {
        let entries = if let Some(user_dirs) = UserDirs::new() {
            vec![
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
            .filter_map(|dir| match dir {
                Some(dir) => {
                    let name = dir.file_name()?.to_string_lossy().to_string();
                    let modified = fs::metadata(dir).ok()?.modified().ok()?;

                    Some(
                        DirectoryEntry::builder()
                            .name(name)
                            .modified(Some(modified))
                            .path(dir)
                            .entry_type(DirectoryEntryType::Directory)
                            .build()
                            .ok()?,
                    )
                }
                None => None,
            })
            .collect()
        } else {
            vec![]
        };

        Self {
            entries,
            selected_index: 0,
            window_size: WindowSize::DefaultSize(40),
            is_open: false,
        }
    }

    pub fn get() -> Arc<Mutex<UserDirectoriesWindow>> {
        USER_DIRECTORIES_WINDOW
            .get_or_init(|| Arc::new(Mutex::new(UserDirectoriesWindow::new())))
            .clone()
    }

    // pub fn toggle(window: Box<dyn Window>) -> Box<dyn Window> {
    // }
}
