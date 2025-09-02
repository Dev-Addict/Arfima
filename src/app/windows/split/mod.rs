mod management;
mod navigation;
mod render;
mod sizing;
mod window_impl;

use ratatui::layout::Direction;

use crate::app::window::{Window, WindowSize, generate_window_id};

use super::DummyWindow;

pub struct SplitWindow {
    id: u32,
    direction: Direction,
    windows: Vec<Box<dyn Window>>,
    focused_index: usize,
    window_size: WindowSize,
}

impl SplitWindow {
    pub fn new(direction: Direction, windows: Vec<Box<dyn Window>>) -> Self {
        Self {
            id: generate_window_id(),
            direction,
            windows,
            focused_index: 0,
            window_size: WindowSize::Default,
        }
    }

    pub fn with_window_size(
        direction: Direction,
        windows: Vec<Box<dyn Window>>,
        window_size: WindowSize,
    ) -> Self {
        Self {
            id: generate_window_id(),
            direction,
            windows,
            focused_index: 0,
            window_size,
        }
    }

    pub fn with_focused_index(
        direction: Direction,
        windows: Vec<Box<dyn Window>>,
        focused_index: usize,
    ) -> Self {
        Self {
            id: generate_window_id(),
            direction,
            focused_index: focused_index.min(windows.len() - 1),
            windows,
            window_size: WindowSize::Default,
        }
    }
}
