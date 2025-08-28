use crate::app::{
    App, InputMode,
    precommand::Precommand,
    widgets::types::InputState,
    windows::{DummyWindow, UserDirectoriesWindow},
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::layout::Direction;

pub fn handle(app: &mut App, key: &KeyEvent) -> bool {
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
            (KeyModifiers::CONTROL, KeyCode::Char('w')) => match precommand {
                Some(Precommand::RepeatWindow(count)) => {
                    let count = *count;
                    *precommand = None;
                    for _ in 0..count {
                        app.next_window();
                    }
                }
                Some(Precommand::Repeat(count)) => {
                    let count = *count;
                    *precommand = Some(Precommand::RepeatWindow(count));
                }
                _ => {
                    *precommand = Some(Precommand::RepeatWindow(1));
                }
            },
            (_, KeyCode::Char('h')) => {
                if let Some(Precommand::RepeatWindow(count)) = precommand {
                    let count = (*count).max(2);
                    *precommand = None;

                    let window = std::mem::replace(&mut app.window, Box::new(DummyWindow));
                    app.window = window.split(Direction::Horizontal, count);

                    return true;
                }

                return false;
            }
            (_, KeyCode::Char('v')) => {
                if let Some(Precommand::RepeatWindow(count)) = precommand {
                    let count = (*count).max(2);
                    *precommand = None;

                    let window = std::mem::replace(&mut app.window, Box::new(DummyWindow));
                    app.window = window.split(Direction::Vertical, count);

                    return true;
                }

                return false;
            }
            (_, KeyCode::Right | KeyCode::Char('j')) => {
                if let Some(Precommand::RepeatWindow(count)) = precommand {
                    let count = *count;
                    *precommand = None;

                    for _ in 0..count {
                        app.next_window();
                    }

                    return true;
                }

                return false;
            }
            (_, KeyCode::Left | KeyCode::Char('k')) => {
                if let Some(Precommand::RepeatWindow(count)) = precommand {
                    let count = *count;
                    *precommand = None;

                    for _ in 0..count {
                        app.prev_window();
                    }

                    return true;
                }

                return false;
            }
            (_, KeyCode::Char('+')) => {
                if let Some(Precommand::RepeatWindow(count)) = precommand {
                    let count = count.cast_signed();
                    *precommand = None;

                    app.adjust_window_size(Direction::Horizontal, count);

                    return true;
                }

                return false;
            }
            (_, KeyCode::Char('-')) => {
                if let Some(Precommand::RepeatWindow(count)) = precommand {
                    let count = count.cast_signed();
                    *precommand = None;

                    app.adjust_window_size(Direction::Horizontal, -count);

                    return true;
                }

                return false;
            }
            (_, KeyCode::Char('=')) => {
                if let Some(Precommand::RepeatWindow(count)) = precommand {
                    let count = count.cast_signed();
                    *precommand = None;

                    app.adjust_window_size(Direction::Vertical, count);

                    return true;
                }

                return false;
            }
            (_, KeyCode::Char('_')) => {
                if let Some(Precommand::RepeatWindow(count)) = precommand {
                    let count = count.cast_signed();
                    *precommand = None;

                    app.adjust_window_size(Direction::Vertical, -count);

                    return true;
                }

                return false;
            }
            (_, KeyCode::Char(':')) => {
                app.input_mode = InputMode::Commanding {
                    state: InputState::default(),
                    current_command: 0,
                    return_state: None,
                };
            }
            (_, KeyCode::Char(' ')) => {
                *precommand = Some(Precommand::Leader);
            }
            (_, KeyCode::Char('n')) => {
                if let Some(Precommand::Leader) = precommand {
                    *precommand = None;

                    let app_window = std::mem::replace(&mut app.window, Box::new(DummyWindow));

                    if let Some(window) = UserDirectoriesWindow::toggle(app_window, &app.config) {
                        app.window = window;
                    } else {
                        app.quit();
                    }

                    return true;
                }

                return false;
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

                    return true;
                }

                return false;
            }
            _ => {
                *precommand = None;
                return false;
            }
        }

        return true;
    }

    false
}
