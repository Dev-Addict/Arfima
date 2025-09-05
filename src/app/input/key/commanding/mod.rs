mod execute_command;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    app::{App, InputMode, widgets::types::InputState},
    command::parse_command,
};

use execute_command::execute_command;

pub fn handle(app: &mut App, key: &KeyEvent) -> bool {
    if let InputMode::Commanding {
        state,
        current_command,
        return_state,
    } = &mut app.input_mode
    {
        match (key.modifiers, key.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => app.quit(),
            (_, KeyCode::Esc) => {
                app.input_mode = InputMode::Normal { precommand: None };
                app.error = None;
            }
            (_, KeyCode::Char(c)) => state.insert_char(c),
            (_, KeyCode::Backspace) => state.remove_char(),
            (KeyModifiers::NONE, KeyCode::Left) => state.left(false),
            (KeyModifiers::SHIFT, KeyCode::Left) => state.left(true),
            (KeyModifiers::NONE, KeyCode::Right) => state.right(false),
            (KeyModifiers::SHIFT, KeyCode::Right) => state.right(true),
            (_, KeyCode::Up | KeyCode::Down) => {
                let diff = if key.code == KeyCode::Up { -1 } else { 1 };

                if *current_command == 0 {
                    if diff == 1 {
                        return true;
                    }
                    *return_state = Some(state.clone());

                    *current_command += diff;

                    if let Some(buffer) = app.command_history.get_from_current(*current_command) {
                        *state = InputState::new(buffer);
                    } else {
                        *current_command -= diff;
                    }
                } else {
                    *current_command += diff;

                    if *current_command == 0 {
                        *state = if let Some(input_state) = return_state {
                            input_state.clone()
                        } else {
                            InputState::new("")
                        };

                        *return_state = None;
                    } else if let Some(buffer) =
                        app.command_history.get_from_current(*current_command)
                    {
                        *state = InputState::new(buffer);
                    } else {
                        *current_command -= diff;
                    }
                }
            }
            (_, KeyCode::Home) => state.set_cursor_position(0),
            (_, KeyCode::End) => state.set_cursor_position(state.buffer().len()),
            (_, KeyCode::Enter) => {
                if state.buffer().is_empty() {
                    app.input_mode = InputMode::Normal { precommand: None };

                    return true;
                }

                app.command_history.push(state.buffer().to_string());

                let handled = match parse_command(state.buffer()) {
                    Ok(command) => execute_command(app, command),
                    Err(e) => {
                        app.error = Some(e.into());
                        true
                    }
                };

                app.input_mode = InputMode::Normal { precommand: None };

                return handled;
            }
            _ => {
                return false;
            }
        }

        return true;
    }

    false
}
