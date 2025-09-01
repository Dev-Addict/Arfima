use crate::{
    app::{App, InputMode},
    command::{BooleanOption, Command, SetCommand, SetOption},
};

pub fn execute_command(app: &mut App, command: Command) -> bool {
    match command {
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
    }

    true
}
