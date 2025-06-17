use std::sync::mpsc::Sender;

use crossterm::event::Event;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
};

use crate::app::{App, AppEvent, InputMode, Result, window::Window};

use super::DummyWindow;

pub struct Split {
    direction: Direction,
    windows: Vec<Box<dyn Window>>,
    focused_index: usize,
}

impl Split {
    pub fn new(direction: Direction, windows: Vec<Box<dyn Window>>) -> Self {
        Self {
            direction,
            windows,
            focused_index: 0,
        }
    }
}

impl Window for Split {
    fn render(&self, app: &App, frame: &mut Frame, area: Rect, focused: bool) {
        let layout = Layout::default()
            .direction(self.direction)
            .constraints(vec![Constraint::Fill(1); self.windows.len()])
            .split(area);

        for (i, window) in self.windows.iter().enumerate() {
            window.render(app, frame, layout[i], focused && self.focused_index == i);
        }
    }

    fn handle_event(
        &mut self,
        input_mode: &InputMode,
        event: &Event,
        focused: bool,
        event_tx: &Sender<AppEvent>,
        mut handled: bool,
    ) -> bool {
        for (i, window) in self.windows.iter_mut().enumerate() {
            if window.handle_event(
                input_mode,
                event,
                focused && self.focused_index == i,
                event_tx,
                handled,
            ) {
                handled = true;
            }
        }

        handled
    }

    fn reset(&mut self) -> Result<()> {
        for window in self.windows.iter_mut() {
            window.reset()?;
        }

        Ok(())
    }

    fn split(self: Box<Self>, direction: Direction) -> Box<dyn Window> {
        let mut this = *self;

        if let Some(window) = this.windows.get_mut(this.focused_index) {
            let focused_window = std::mem::replace(window, Box::new(DummyWindow));
            *window = focused_window.split(direction);
        }

        Box::new(this)
    }

    fn next_window(&mut self) -> bool {
        if let Some(window) = self.windows.get_mut(self.focused_index) {
            if !window.next_window() {
                if self.focused_index < self.windows.len().saturating_sub(1) {
                    self.focused_index = self.focused_index.saturating_add(1);
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

    fn abs_next_window(&mut self) {
        if self.windows.is_empty() {
            return;
        }

        self.focused_index = self.focused_index.saturating_add(1) % self.windows.len();
        if let Some(window) = self.windows.get_mut(self.focused_index) {
            window.abs_next_window();
        }
    }

    fn prev_window(&mut self) -> bool {
        if let Some(window) = self.windows.get_mut(self.focused_index) {
            if !window.prev_window() {
                if self.focused_index > 0 {
                    self.focused_index = self.focused_index.saturating_sub(1);
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

    fn abs_prev_window(&mut self) {
        if self.focused_index == 0 {
            self.focused_index = self.windows.len().saturating_sub(1);
        } else {
            self.focused_index = self.focused_index.saturating_sub(1);
        }

        if let Some(window) = self.windows.get_mut(self.focused_index) {
            window.abs_prev_window();
        }
    }

    fn quit_focused_window(self: Box<Self>) -> Option<Box<dyn Window>> {
        let mut this = *self;

        if this.windows.is_empty() {
            return None;
        }

        if let Some(window) = this.windows.get_mut(this.focused_index) {
            let old_window = std::mem::replace(window, Box::new(DummyWindow));
            if let Some(new_child) = old_window.quit_focused_window() {
                this.windows[this.focused_index] = new_child;
                return Some(Box::new(this));
            }
        }

        this.windows.remove(this.focused_index);

        if this.windows.is_empty() {
            return None;
        } else if this.windows.len() == 1 {
            return Some(this.windows.remove(0));
        } else if this.focused_index >= this.windows.len() {
            this.focused_index = this.windows.len() - 1;
        }

        Some(Box::new(this))
    }
}
