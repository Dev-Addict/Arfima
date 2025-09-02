use ratatui::layout::Direction;

use crate::app::window::WindowSize;

use super::SplitWindow;

pub fn adjust_window_size(
    split: &mut SplitWindow,
    direction: Direction,
    adjustment: isize,
    parent: Option<(&Direction, usize)>,
) -> bool {
    let windows_len = split.windows.len();

    if let Some(window) = split.windows.get_mut(split.focused_index) {
        if window.adjust_window_size(direction, adjustment, Some((&split.direction, windows_len))) {
            return true;
        }
    }

    if let Some((d, windows)) = parent {
        if d == &direction {
            split.window_size = match split.window_size {
                WindowSize::Default => {
                    WindowSize::Adjusted(adjustment.saturating_mul(windows.cast_signed()))
                }
                WindowSize::DefaultSize(size) => {
                    WindowSize::AdjustedSize(size, adjustment.saturating_mul(windows.cast_signed()))
                }
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
