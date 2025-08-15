use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    app::{App, InputMode},
    utils::{
        parse_command,
        process_command::{OptionName, SetCommand},
    },
};

// TODO: Add History for commands

pub fn handle(app: &mut App, key: &KeyEvent) -> bool {
    if let InputMode::Commanding { state } = &mut app.input_mode {
        match (key.modifiers, key.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => app.quit(),
            (_, KeyCode::Esc) => {
                app.input_mode = InputMode::Normal { precommand: None };
                app.error = None;
            }
            (_, KeyCode::Char(c)) => state.insert_char(c),
            (_, KeyCode::Backspace) => state.remove_char(),
            (_, KeyCode::Left) => state.left(),
            (_, KeyCode::Right) => state.right(),
            (_, KeyCode::Home) => state.set_cursor_position(0),
            (_, KeyCode::End) => state.set_cursor_position(state.buffer().len()),
            (_, KeyCode::Enter) => {
                if state.buffer().is_empty() {
                    app.input_mode = InputMode::Normal { precommand: None };

                    return true;
                }

                match parse_command(state.buffer()) {
                    Ok(command) => {
                        let active = match command {
                            SetCommand::Enable(_) => true,
                            SetCommand::Disable(_) => false,
                            SetCommand::Toggle(_) => !app.config.number().active(),
                        };

                        match command {
                            SetCommand::Enable(option)
                            | SetCommand::Disable(option)
                            | SetCommand::Toggle(option) => match option {
                                OptionName::Number => {
                                    app.config.mut_number().set_active(active);
                                }
                                OptionName::RelativeNumber => {
                                    app.config.mut_number().set_relative(active);

                                    if active {
                                        app.config.mut_number().set_active(true);
                                    }
                                }
                            },
                        }

                        app.error = None;
                    }
                    Err(e) => app.error = Some(e.into()),
                };

                app.input_mode = InputMode::Normal { precommand: None };
            }
            _ => {
                return false;
            }
        }

        return true;
    }

    false
}
