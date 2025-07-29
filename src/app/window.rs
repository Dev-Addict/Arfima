use std::sync::mpsc::Sender;

use crossterm::event::Event;
use ratatui::{
    Frame,
    layout::{Direction, Rect},
};

use super::{App, AppEvent, InputMode, Result};

#[derive(Clone)]
pub enum WindowSize {
    Default,
    /// DefaultSize is for windows that start with a size in mind
    /// Example in neovim would be the explorer window that usually takes way less space than half
    /// of the screen
    DefaultSize(usize),
    /// These are for Default windows after their window size is adjusted
    /// the type is isize allowing positive or negative numbers they won't keep in mind what the
    /// size of the window is they will calculate based on default behavior and then this number
    Adjusted(isize),
    /// These are fro DefaultSize windows after their window size is adjusted same as Adjusted but
    /// keeping the default size assigned to window in mind
    AdjustedSize(usize, isize),
}

pub trait Window {
    fn render(&self, app: &App, frame: &mut Frame, area: Rect, focused: bool);
    fn handle_event(
        &mut self,
        input_mode: &InputMode,
        event: &Event,
        focused: bool,
        event_tx: &Sender<AppEvent>,
        handled: bool,
    ) -> bool;
    fn split(self: Box<Self>, direction: Direction, count: usize) -> Box<dyn Window>;

    fn reset(&mut self) -> Result<()> {
        Ok(())
    }

    fn next_window(&mut self) -> bool {
        false
    }

    fn abs_next_window(&mut self) {}

    fn prev_window(&mut self) -> bool {
        false
    }

    fn abs_prev_window(&mut self) {}

    fn quit_focused_window(self: Box<Self>) -> Option<Box<dyn Window>> {
        None
    }

    fn get_window_size(&self) -> &WindowSize {
        &WindowSize::Default
    }

    /// _parent indicates whether the window has a parent
    /// The return value indicates whether the adjustment was handled
    fn adjust_window_size(
        &mut self,
        _direction: Direction,
        _adjustment: isize,
        _parent: Option<&Direction>,
    ) -> bool {
        false
    }
}
