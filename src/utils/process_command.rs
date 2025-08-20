use std::fmt::Display;

// TODO: restructure into a more organized format

use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace1,
    combinator::{map, opt},
};

#[derive(Debug)]
pub enum Error {
    FailedToParseError(nom::Err<nom::error::Error<String>>),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FailedToParseError(e) => write!(f, "Failed to parse the command: {e}"),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

pub enum OptionName {
    Number,
    RelativeNumber,
}

pub enum SetCommand {
    Enable(OptionName),
    Disable(OptionName),
    Toggle(OptionName),
}

pub struct QuitCommand {
    all: bool,
}

impl QuitCommand {
    pub fn all(&self) -> bool {
        self.all
    }
}

pub enum Command {
    Set(SetCommand),
    Quit(QuitCommand),
}

fn parse_option_name(input: &str) -> IResult<&str, OptionName> {
    alt((
        map(alt((tag("number"), tag("nu"))), |_| OptionName::Number),
        map(alt((tag("relativenumber"), tag("rnu"))), |_| {
            OptionName::RelativeNumber
        }),
    ))(input)
}

fn parse_set_command(input: &str) -> IResult<&str, SetCommand> {
    let (input, _) = alt((tag("set"), tag("se")))(input)?;
    let (input, _) = multispace1(input)?;

    let (input, is_no) = opt(tag("no"))(input)?;
    let (input, name) = parse_option_name(input)?;
    let (input, is_toggle) = opt(tag("!"))(input)?;

    let command = match (is_no, is_toggle) {
        (Some(_), None) => SetCommand::Disable(name),
        (None, None) => SetCommand::Enable(name),
        (None, Some(_)) => SetCommand::Toggle(name),
        _ => {
            return Err(nom::Err::Failure(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Tag,
            )));
        }
    };

    Ok((input, command))
}

fn parse_quit_command(input: &str) -> IResult<&str, QuitCommand> {
    alt((
        map(alt((tag("qa"), tag("quitall"))), |_| QuitCommand {
            all: true,
        }),
        map(alt((tag("q"), tag("quit"))), |_| QuitCommand { all: false }),
    ))(input)
}

fn parse_any_command(input: &str) -> IResult<&str, Command> {
    alt((
        map(parse_set_command, Command::Set),
        map(parse_quit_command, Command::Quit),
    ))(input)
}

pub fn parse_command(input: &str) -> Result<Command> {
    match parse_any_command(input) {
        Ok((_, command)) => Ok(command),
        Err(e) => Err(Error::FailedToParseError(e.to_owned())),
    }
}
