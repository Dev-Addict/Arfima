use std::sync::mpsc::Sender;

use crossterm::event::Event;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
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

    fn reset(&mut self) -> Result<()> {
        Ok(())
    }
}

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
    ) {
        for (i, window) in self.windows.iter_mut().enumerate() {
            window.handle_event(
                input_mode,
                event,
                focused && self.focused_index == i,
                event_tx,
            );
        }
    }

    fn reset(&mut self) -> Result<()> {
        for window in self.windows.iter_mut() {
            window.reset()?;
        }

        Ok(())
    }
}
