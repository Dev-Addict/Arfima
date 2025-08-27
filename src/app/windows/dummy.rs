use std::{
    any::{Any, TypeId},
    sync::mpsc::Sender,
};

use crossterm::event::Event;
use ratatui::{
    Frame,
    layout::{Direction, Rect},
};

use crate::app::{App, AppEvent, Error, InputMode, window::Window};

pub struct DummyWindow;

impl Window for DummyWindow {
    fn id(&self) -> u32 {
        0
    }

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

    fn includes(&self, id: u32) -> bool {
        id == 0
    }

    fn open(
        self: Box<Self>,
        _: std::path::PathBuf,
        _: bool,
    ) -> (Box<dyn Window>, Option<crate::app::Error>) {
        (self, Some(Error::NotADummy))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn includes_type_id(&self, type_id: TypeId) -> Option<u32> {
        if type_id == TypeId::of::<DummyWindow>() {
            Some(0)
        } else {
            None
        }
    }
}
