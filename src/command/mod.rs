mod error;
mod quit;
pub mod result;
mod save;
mod set;

pub use error::Error;
pub use quit::QuitCommand;
use quit::parse_quit_command;
use result::Result;
use save::parse_save_command;
use set::parse_set_command;
pub use set::{BooleanOption, SetCommand, SetOption};

use nom::{IResult, branch::alt, combinator::map};

pub enum Command {
    Set(SetCommand),
    Quit(QuitCommand),
    Save(()),
}

fn parse_any_command(input: &str) -> IResult<&str, Command> {
    alt((
        map(parse_set_command, Command::Set),
        map(parse_quit_command, Command::Quit),
        map(parse_save_command, Command::Save),
    ))(input)
}

pub fn parse_command(input: &str) -> Result<Command> {
    match parse_any_command(input) {
        Ok((_, command)) => Ok(command),
        Err(e) => Err(Error::FailedToParseError(e.to_owned())),
    }
}
