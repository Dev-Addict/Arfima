mod error;
mod input;
mod input_mode;
mod precommand;
mod result;
mod ui;
pub mod widgets;
mod window;
mod windows;

use std::sync::mpsc::{Receiver, Sender, channel};

use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
pub use error::Error;
use input::handle_key_event;
pub use input_mode::InputMode;
use precommand::Precommand;
use ratatui::{DefaultTerminal, layout::Direction};
pub use result::Result;
use ui::render_ui;
use window::{Split, Window};
use windows::FileManagerWindow;

pub struct App {
    running: bool,
    input_mode: InputMode,
    error: Option<Error>,
    window: Box<dyn Window>,
    event_rx: Receiver<AppEvent>,
}

pub enum AppEvent {
    UpdatePrecommand(Option<Precommand>),
    SetError(Option<Error>),
    UpdateInputMode(InputMode),
    Reset,
}

impl App {
    pub fn new(directory: &str) -> Result<(Self, Sender<AppEvent>)> {
        let (tx, rx) = channel();

        Ok((
            Self {
                running: false,
                input_mode: InputMode::Normal { precommand: None },
                error: None,
                window: Box::new(Split::new(
                    Direction::Horizontal,
                    vec![
                        Box::new(FileManagerWindow::new(directory)?),
                        Box::new(FileManagerWindow::new(directory)?),
                    ],
                )),
                event_rx: rx,
            },
            tx,
        ))
    }

    pub fn reset(&mut self) -> Result<()> {
        self.running = true;
        self.input_mode = InputMode::Normal { precommand: None };
        self.error = None;
        self.window.reset()?;

        Ok(())
    }

    pub fn run(mut self, mut terminal: DefaultTerminal, event_tx: &Sender<AppEvent>) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| render_ui(&mut self, frame))?;

            self.handle_crossterm_events(event_tx)?;

            while let Ok(event) = self.event_rx.try_recv() {
                self.handle_app_events(event)?;
            }
        }
        Ok(())
    }

    fn handle_crossterm_events(&mut self, event_tx: &Sender<AppEvent>) -> Result<()> {
        let event = event::read()?;

        match event {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }

        self.window
            .handle_event(&self.input_mode, &event, true, event_tx);

        Ok(())
    }

    fn handle_app_events(&mut self, event: AppEvent) -> Result<()> {
        match event {
            AppEvent::UpdatePrecommand(new_precommand) => {
                if let InputMode::Normal { precommand } = &mut self.input_mode {
                    *precommand = new_precommand;
                }
            }
            AppEvent::SetError(e) => {
                self.error = e;
            }
            AppEvent::UpdateInputMode(input_mode) => {
                self.input_mode = input_mode;
            }
            AppEvent::Reset => {
                self.reset()?;
            }
        }

        Ok(())
    }

    fn on_key_event(&mut self, key: KeyEvent) {
        handle_key_event(self, key);
    }

    fn quit(&mut self) {
        self.running = false;
    }
}
