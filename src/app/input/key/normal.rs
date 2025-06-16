use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{App, InputMode, precommand::Precommand};

pub fn handle(app: &mut App, key: KeyEvent) {
    if let InputMode::Normal { precommand } = &mut app.input_mode {
        match (key.modifiers, key.code) {
            (_, KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => app.quit(),
            (_, KeyCode::Esc) => {
                if let Err(e) = app.reset() {
                    app.error = Some(e);
                }
            }
            (KeyModifiers::CONTROL, KeyCode::Char('h')) => {
                app.input_mode = InputMode::Help { selected_index: 0 };
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
            _ => {}
        }
    }
}
