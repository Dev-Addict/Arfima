use std::fmt::Display;

// TODO: restructure into a more organized format

use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, multispace1},
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

pub enum BooleanOption {
    Number,
    RelativeNumber,
}

pub enum SetOption {
    HistorySize(usize),
}

pub enum SetCommand {
    Enable(BooleanOption),
    Disable(BooleanOption),
    Toggle(BooleanOption),
    Set(SetOption),
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
    Save(()),
}

fn parse_history_size_assignment(input: &str) -> IResult<&str, SetOption> {
    let (input, _) = tag("history_size")(input)?;
    let (input, _) = alt((tag("="), multispace1))(input)?;
    let (input, digits) = digit1(input)?;

    let value: usize = digits.parse().map_err(|_| {
        nom::Err::Failure(nom::error::Error::new(input, nom::error::ErrorKind::Digit))
    })?;

    Ok((input, SetOption::HistorySize(value)))
}

fn parse_boolean_option(input: &str) -> IResult<&str, BooleanOption> {
    alt((
        map(alt((tag("number"), tag("nu"))), |_| BooleanOption::Number),
        map(alt((tag("relativenumber"), tag("rnu"))), |_| {
            BooleanOption::RelativeNumber
        }),
    ))(input)
}

fn parse_boolean_set_command(input: &str) -> IResult<&str, SetCommand> {
    let (input, is_no) = opt(tag("no"))(input)?;
    let (input, name) = parse_boolean_option(input)?;
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

fn parse_set_command(input: &str) -> IResult<&str, SetCommand> {
    let (input, _) = alt((tag("set"), tag("se")))(input)?;
    let (input, _) = multispace1(input)?;

    alt((
        map(parse_history_size_assignment, SetCommand::Set),
        parse_boolean_set_command,
    ))(input)
}

fn parse_quit_command(input: &str) -> IResult<&str, QuitCommand> {
    alt((
        map(alt((tag("qa"), tag("quitall"))), |_| QuitCommand {
            all: true,
        }),
        map(alt((tag("q"), tag("quit"))), |_| QuitCommand { all: false }),
    ))(input)
}

fn parse_save_command(input: &str) -> IResult<&str, ()> {
    map(alt((tag("s"), tag("save"))), |_| ())(input)
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
