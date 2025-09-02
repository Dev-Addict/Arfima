use std::any::{Any, TypeId};

use crossbeam::channel::Sender;
use crossterm::event::Event;
use ratatui::{
    Frame,
    layout::{Direction, Rect},
};

use crate::{
    app::{
        App, AppEvent, InputMode, Result,
        window::{Window, WindowSize},
    },
    config::Config,
};

use super::{SplitWindow, management, navigation, render, sizing};

impl Window for SplitWindow {
    fn id(&self) -> u32 {
        self.id
    }

    fn render(&self, app: &App, frame: &mut Frame, area: Rect, focused: bool) {
        render::render(self, app, frame, area, focused);
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

    fn reset(&mut self, config: &Config) -> Result<()> {
        for window in self.windows.iter_mut() {
            window.reset(config)?;
        }

        Ok(())
    }

    fn split(self: Box<Self>, direction: Direction, count: usize) -> Box<dyn Window> {
        management::split(*self, direction, count)
    }

    fn next_window(&mut self) -> bool {
        navigation::next_window(self)
    }

    fn abs_next_window(&mut self) {
        navigation::abs_next_window(self)
    }

    fn prev_window(&mut self) -> bool {
        navigation::prev_window(self)
    }

    fn abs_prev_window(&mut self) {
        navigation::abs_prev_window(self)
    }

    fn quit_focused_window(self: Box<Self>) -> Option<Box<dyn Window>> {
        management::quit_focused_window(*self)
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
        sizing::adjust_window_size(self, direction, adjustment, parent)
    }

    fn includes(&self, id: u32) -> bool {
        if self.id == id {
            return true;
        }

        for window in &self.windows {
            if window.includes(id) {
                return true;
            }
        }

        false
    }

    fn remove(self: Box<Self>, id: u32) -> Option<Box<dyn Window>> {
        management::remove(*self, id)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn open(
        self: Box<Self>,
        path: std::path::PathBuf,
        new: bool,
    ) -> (Box<dyn Window>, Option<crate::app::Error>) {
        management::open(*self, path, new)
    }

    fn includes_type_id(&self, type_id: TypeId) -> Option<u32> {
        if type_id == TypeId::of::<SplitWindow>() {
            Some(self.id)
        } else {
            for window in self.windows.iter() {
                if let Some(id) = window.includes_type_id(type_id) {
                    return Some(id);
                }
            }

            None
        }
    }
}
