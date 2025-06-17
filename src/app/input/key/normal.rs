use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::layout::Direction;

use crate::app::{App, InputMode, precommand::Precommand, windows::DummyWindow};

pub fn handle(app: &mut App, key: &KeyEvent) {
    if let InputMode::Normal { precommand } = &mut app.input_mode {
        match (key.modifiers, key.code) {
            (_, KeyCode::Char('q')) => app.quit_focused_window(),
            (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => app.quit(),
            (_, KeyCode::Esc) => {
                if let Err(e) = app.reset() {
                    app.error = Some(e);
                }
            }
            (KeyModifiers::CONTROL, KeyCode::Char('h')) => {
                app.input_mode = InputMode::Help { selected_index: 0 };
            }
            (KeyModifiers::CONTROL, KeyCode::Char('w')) => {
                if let Some(Precommand::Window) = precommand {
                    *precommand = None;

                    app.next_window();
                } else {
                    *precommand = Some(Precommand::Window);
                }
            }
            (_, KeyCode::Char('h')) => {
                if let Some(Precommand::Window) = precommand {
                    *precommand = None;

                    let window = std::mem::replace(&mut app.window, Box::new(DummyWindow));
                    app.window = window.split(Direction::Horizontal);
                }
            }
            (_, KeyCode::Char('v')) => {
                if let Some(Precommand::Window) = precommand {
                    *precommand = None;

                    let window = std::mem::replace(&mut app.window, Box::new(DummyWindow));
                    app.window = window.split(Direction::Vertical);
                }
            }
            (_, KeyCode::Right | KeyCode::Char('j')) => {
                if let Some(Precommand::Window) = precommand {
                    *precommand = None;

                    app.next_window();
                }
            }
            (_, KeyCode::Left | KeyCode::Char('k')) => {
                if let Some(Precommand::Window) = precommand {
                    *precommand = None;

                    app.prev_window();
                }
            }
            (_, KeyCode::Char(c)) => {
                if c.is_ascii_digit() {
                    let digit = c.to_digit(10).unwrap() as usize;

                    match precommand {
                        Some(precommand) => match precommand {
                            Precommand::Repeat(repeat) => *repeat = *repeat * 10 + digit,
                            _ => *precommand = Precommand::Repeat(digit),
                        },
                        None => *precommand = Some(Precommand::Repeat(digit)),
                    }
                }
            }
            _ => *precommand = None,
        }
    }
}
