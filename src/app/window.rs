use std::sync::mpsc::Sender;

use crossterm::event::Event;
use ratatui::{
    Frame,
    layout::{Direction, Rect},
};

use super::{App, AppEvent, InputMode, Result};

pub trait Window {
    fn render(&self, app: &App, frame: &mut Frame, area: Rect, focused: bool);
    fn handle_event(
        &mut self,
        input_mode: &InputMode,
        event: &Event,
        focused: bool,
        event_tx: &Sender<AppEvent>,
    );
    fn split(self: Box<Self>, direction: Direction) -> Box<dyn Window>;

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
}
