use super::SplitWindow;

pub fn next_window(split: &mut SplitWindow) -> bool {
    if let Some(window) = split.windows.get_mut(split.focused_index) {
        if !window.next_window() {
            if split.focused_index < split.windows.len().saturating_sub(1) {
                split.focused_index = split.focused_index.saturating_add(1);
                true
            } else {
                false
            }
        } else {
            true
        }
    } else {
        false
    }
}

pub fn abs_next_window(split: &mut SplitWindow) {
    if split.windows.is_empty() {
        return;
    }

    split.focused_index = split.focused_index.saturating_add(1) % split.windows.len();
    if let Some(window) = split.windows.get_mut(split.focused_index) {
        window.abs_next_window();
    }
}

pub fn prev_window(split: &mut SplitWindow) -> bool {
    if let Some(window) = split.windows.get_mut(split.focused_index) {
        if !window.prev_window() {
            if split.focused_index > 0 {
                split.focused_index = split.focused_index.saturating_sub(1);
                true
            } else {
                false
            }
        } else {
            true
        }
    } else {
        false
    }
}

pub fn abs_prev_window(split: &mut SplitWindow) {
    if split.focused_index == 0 {
        split.focused_index = split.windows.len().saturating_sub(1);
    } else {
        split.focused_index = split.focused_index.saturating_sub(1);
    }

    if let Some(window) = split.windows.get_mut(split.focused_index) {
        window.abs_prev_window();
    }
}
