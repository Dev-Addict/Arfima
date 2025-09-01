mod input;

use std::{
    any::{Any, TypeId},
    path::PathBuf,
    sync::LazyLock,
};

use crossbeam::channel::Sender;
use crossterm::event::Event;
use directories::UserDirs;
use ratatui::{
    Frame,
    layout::{Direction, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::Block,
};

use crate::{
    app::{
        App, AppEvent, Error, InputMode, Result,
        widgets::draw_minimal_entries_table,
        window::{Window, WindowSize, generate_window_id},
        windows::{FileManagerWindow, Split},
    },
    config::Config,
    directory_entry::DirectoryEntry,
};

use input::handle_event;

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
            Some(Box::new(Split::with_window_size(
                Direction::Horizontal,
                vec![Box::new(Self::new(config)), window],
                WindowSize::Default,
            )))
        }
    }
}

impl Window for CommonEntriesWindow {
    fn id(&self) -> u32 {
        *COMMON_ENTRIES_WINDOW_ID
    }

    fn render(&self, _app: &App, frame: &mut Frame, area: Rect, focused: bool) {
        let mut block = Block::bordered().title(
            Line::from(vec![
                Span::styled("", Style::default()),
                Span::styled(" Arfima ", Style::default().reversed()),
                Span::styled("", Style::default()),
            ])
            .bold(),
        );

        if focused {
            block = block.border_style(Style::default().fg(Color::Cyan));
        }

        draw_minimal_entries_table(frame, area, &self.entries, self.selected_index, block);
    }

    fn handle_event(
        &mut self,
        input_mode: &InputMode,
        event: &Event,
        focused: bool,
        event_tx: &Sender<AppEvent>,
        handled: bool,
    ) -> bool {
        if !focused || handled {
            return false;
        }

        handle_event(self, input_mode, event, event_tx)
    }

    fn reset(&mut self, config: &Config) -> Result<()> {
        self.entries = Self::entries(config);
        self.selected_index = self.selected_index.min(self.entries.len() - 1);

        Ok(())
    }

    fn split(self: Box<Self>, _: Direction, _: usize) -> Box<dyn Window> {
        self
    }

    fn get_window_size(&self) -> &WindowSize {
        &self.window_size
    }

    fn adjust_window_size(
        &mut self,
        direction: Direction,
        adjustment: isize,
        parent: Option<(&Direction, usize)>,
    ) -> bool {
        if let Some((d, windows)) = parent {
            if d == &direction {
                self.window_size = match self.window_size {
                    WindowSize::Default => {
                        WindowSize::Adjusted(adjustment.saturating_mul(windows.cast_signed()))
                    }
                    WindowSize::DefaultSize(size) => WindowSize::AdjustedSize(
                        size,
                        adjustment.saturating_mul(windows.cast_signed()),
                    ),
                    WindowSize::Adjusted(prev) => WindowSize::Adjusted(
                        prev.saturating_add(adjustment.saturating_mul(windows.cast_signed())),
                    ),
                    WindowSize::AdjustedSize(size, prev) => WindowSize::AdjustedSize(
                        size,
                        prev.saturating_add(adjustment.saturating_mul(windows.cast_signed())),
                    ),
                };

                return true;
            }
        }

        false
    }

    fn includes(&self, id: u32) -> bool {
        *COMMON_ENTRIES_WINDOW_ID == id
    }

    fn open(self: Box<Self>, path: PathBuf, _: bool) -> (Box<dyn Window>, Option<Error>) {
        let file_manager = match FileManagerWindow::new(path.to_string_lossy().as_ref()) {
            Ok(window) => window,
            Err(e) => return (self, Some(e)),
        };

        (
            Box::new(Split::with_focused_index(
                Direction::Horizontal,
                vec![self, Box::new(file_manager)],
                1,
            )),
            None,
        )
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn includes_type_id(&self, type_id: TypeId) -> Option<u32> {
        if type_id == TypeId::of::<CommonEntriesWindow>() {
            Some(*COMMON_ENTRIES_WINDOW_ID)
        } else {
            None
        }
    }
}
