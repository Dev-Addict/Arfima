use ratatui::{
    Frame,
    layout::{Direction, Rect},
};

use crate::app::{App, window::WindowSize};

use super::SplitWindow;

pub fn render(split: &SplitWindow, app: &App, frame: &mut Frame, area: Rect, focused: bool) {
    let mut areas = vec![area; split.windows.len()];
    let mut window_sizes = vec![0_usize; split.windows.len()];
    let mut constant_width = 0;
    let mut all_adjustments = 0;
    let mut sized_windows = 0;

    for (i, window) in split.windows.iter().enumerate() {
        match window.get_window_size() {
            WindowSize::Adjusted(adjustment) => {
                all_adjustments += adjustment;
            }
            WindowSize::DefaultSize(size) => {
                constant_width += size;
                window_sizes[i] = *size;
                sized_windows += 1;
            }
            WindowSize::AdjustedSize(size, adjustment) => {
                let mut size = if *adjustment < 0 {
                    size.saturating_sub(adjustment.unsigned_abs())
                } else {
                    size.saturating_add(*adjustment as usize)
                };

                if size < 3 {
                    size = 3;
                }

                constant_width += size;
                window_sizes[i] = size;
                sized_windows += 1;
            }
            WindowSize::Default => {}
        }
    }

    let to_divide = match split.direction {
        Direction::Vertical => area.height,
        Direction::Horizontal => area.width,
    } as usize
        - constant_width;
    let default_size = if all_adjustments >= 0 {
        (to_divide / (split.windows.len() - sized_windows))
            - (all_adjustments.unsigned_abs() / (split.windows.len() - sized_windows))
    } else {
        (to_divide / (split.windows.len() - sized_windows))
            + (all_adjustments.unsigned_abs() / (split.windows.len() - sized_windows))
    };
    let mut remainder = (to_divide % (split.windows.len() - sized_windows)) as isize
        - (all_adjustments % (split.windows.len() - sized_windows) as isize);

    for (i, window) in split.windows.iter().enumerate() {
        match window.get_window_size() {
            WindowSize::Default => {
                window_sizes[i] = if remainder > 0 {
                    remainder -= 1;
                    default_size.saturating_add(1)
                } else if remainder < 0 {
                    remainder += 1;
                    default_size.saturating_sub(1)
                } else {
                    default_size
                };
            }
            WindowSize::Adjusted(adjustment) => {
                let size = if *adjustment > 0 {
                    default_size.saturating_add(adjustment.unsigned_abs())
                } else {
                    default_size.saturating_sub(adjustment.unsigned_abs())
                };

                window_sizes[i] = if remainder > 0 {
                    remainder -= 1;
                    size.saturating_add(1)
                } else if remainder < 0 {
                    remainder += 1;
                    size.saturating_sub(1)
                } else {
                    size
                };
            }
            _ => {}
        }
    }

    let mut accumulated_z = 0;

    for (i, area) in areas.iter_mut().enumerate() {
        match split.direction {
            Direction::Vertical => {
                area.height = window_sizes[i].try_into().unwrap_or(0);
                area.y += accumulated_z;
                accumulated_z += area.height;
            }
            Direction::Horizontal => {
                area.width = window_sizes[i].try_into().unwrap_or(0);
                area.x += accumulated_z;
                accumulated_z += area.width;
            }
        }
    }

    for (i, window) in split.windows.iter().enumerate() {
        window.render(app, frame, areas[i], focused && split.focused_index == i);
    }
}
