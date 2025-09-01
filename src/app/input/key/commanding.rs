use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    app::{App, InputMode, widgets::types::InputState},
    command::{BooleanOption, Command, SetCommand, SetOption, parse_command},
};

// TODO: Handle the commands somewhere else

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
            (_, KeyCode::Left) => state.left(),
            (_, KeyCode::Right) => state.right(),
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

                match parse_command(state.buffer()) {
                    Ok(command) => match command {
                        Command::Set(command) => {
                            let active = match command {
                                SetCommand::Enable(_) => true,
                                SetCommand::Disable(_) => false,
                                SetCommand::Toggle(_) => !app.config.number().active(),
                                SetCommand::Set(option) => match option {
                                    SetOption::HistorySize(size) => {
                                        app.config.mut_history().set_size(size);

                                        app.command_history.set_size(size);

                                        app.input_mode = InputMode::Normal { precommand: None };

                                        return true;
                                    }
                                },
                            };

                            match command {
                                SetCommand::Enable(option)
                                | SetCommand::Disable(option)
                                | SetCommand::Toggle(option) => match option {
                                    BooleanOption::Number => {
                                        app.config.mut_number().set_active(active);
                                    }
                                    BooleanOption::RelativeNumber => {
                                        app.config.mut_number().set_relative(active);

                                        if active {
                                            app.config.mut_number().set_active(true);
                                        }
                                    }
                                },
                                SetCommand::Set(_) => {
                                    app.input_mode = InputMode::Normal { precommand: None };

                                    return false;
                                }
                            }

                            app.error = None;
                        }
                        Command::Quit(command) => {
                            if command.all() {
                                app.quit();
                            } else {
                                app.quit_focused_window();
                            }
                        }
                        Command::Save(_) => {
                            if let Err(e) = app.config.save() {
                                app.error = Some(e.into())
                            }
                        }
                    },
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
