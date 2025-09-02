use std::any::TypeId;

use ratatui::layout::Direction;

use crate::app::{window::Window, windows::FileManagerWindow};

use super::DummyWindow;
use super::SplitWindow;

pub fn split(mut split: SplitWindow, direction: Direction, count: usize) -> Box<dyn Window> {
    if let Some(window) = split.windows.get_mut(split.focused_index) {
        let focused_window = std::mem::replace(window, Box::new(DummyWindow));
        *window = focused_window.split(direction, count);
    }

    Box::new(split)
}

pub fn quit_focused_window(mut split: SplitWindow) -> Option<Box<dyn Window>> {
    if split.windows.is_empty() {
        return None;
    }

    if let Some(window) = split.windows.get_mut(split.focused_index) {
        let old_window = std::mem::replace(window, Box::new(DummyWindow));
        if let Some(new_child) = old_window.quit_focused_window() {
            split.windows[split.focused_index] = new_child;
            return Some(Box::new(split));
        }
    }

    split.windows.remove(split.focused_index);

    if split.windows.is_empty() {
        return None;
    } else if split.windows.len() == 1 {
        return Some(split.windows.remove(0));
    } else if split.focused_index >= split.windows.len() {
        split.focused_index = split.windows.len() - 1;
    }

    Some(Box::new(split))
}

pub fn remove(mut split: SplitWindow, id: u32) -> Option<Box<dyn Window>> {
    if split.windows.is_empty() {
        return None;
    }

    if split.id == id {
        return None;
    }

    let mut removable_index = None;
    let mut includes_index = None;

    for (i, window) in split.windows.iter().enumerate() {
        if window.id() == id {
            removable_index = Some(i);
        }

        if window.includes(id) {
            includes_index = Some(i)
        }
    }

    if let Some(i) = removable_index {
        split.windows.remove(i);

        if split.windows.is_empty() {
            return None;
        } else if split.windows.len() == 1 {
            return Some(split.windows.remove(0));
        } else if split.focused_index >= split.windows.len() {
            split.focused_index = split.windows.len() - 1;
        }
    } else if let Some(i) = includes_index {
        if let Some(window) = split.windows.get_mut(i) {
            let old_window = std::mem::replace(window, Box::new(DummyWindow));
            if let Some(new_child) = old_window.remove(id) {
                split.windows[i] = new_child;
            }
        }
    }

    Some(Box::new(split))
}

pub fn open(
    mut split: SplitWindow,
    path: std::path::PathBuf,
    new: bool,
) -> (Box<dyn Window>, Option<crate::app::Error>) {
    if !new && let Some(id) = split.includes_type_id(TypeId::of::<FileManagerWindow>()) {
        let mut openable_index = None;
        let mut includes_index = None;

        for (i, window) in split.windows.iter().enumerate() {
            if window.id() == id {
                openable_index = Some(i);
            }

            if window.includes(id) {
                includes_index = Some(i)
            }
        }

        if let Some(i) = openable_index {
            if let Some(window) = split.windows.get_mut(i) {
                let window = std::mem::replace(window, Box::new(DummyWindow));

                split.windows[i] = match FileManagerWindow::with_id_and_window_size(
                    path.to_string_lossy().as_ref(),
                    window.id(),
                    window.get_window_size().to_owned(),
                ) {
                    Ok(window) => Box::new(window),
                    Err(e) => {
                        split.windows[i] = window;
                        return (Box::new(split), Some(e));
                    }
                };
                split.focused_index = i;
            }
        } else if let Some(i) = includes_index {
            if let Some(window) = split.windows.get_mut(i) {
                let window = std::mem::replace(window, Box::new(DummyWindow));
                let (window, error) = window.open(path, false);
                split.windows[i] = window;

                if let Some(e) = error {
                    return (Box::new(split), Some(e));
                }

                split.focused_index = i;
            }
        }
    } else {
        split.windows.push(
            match FileManagerWindow::new(path.to_string_lossy().as_ref()) {
                Ok(window) => Box::new(window),
                Err(e) => {
                    return (Box::new(split), Some(e));
                }
            },
        );

        split.focused_index = split.windows.len() - 1;
    }

    (Box::new(split), None)
}
