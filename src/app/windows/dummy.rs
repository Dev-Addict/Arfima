use std::sync::mpsc::Sender;

use crossterm::event::Event;
use ratatui::{
    Frame,
    layout::{Direction, Rect},
};

use crate::app::{App, AppEvent, InputMode, window::Window};

pub struct DummyWindow;

impl Window for DummyWindow {
    fn render(&self, _: &App, _: &mut Frame, _: Rect, _: bool) {}
    fn handle_event(
        &mut self,
        _: &InputMode,
        _: &Event,
        _: bool,
        _: &Sender<AppEvent>,
        _: bool,
    ) -> bool {
        false
    }
    fn split(self: Box<Self>, _: Direction, _: usize) -> Box<dyn Window> {
        self
    }
}
