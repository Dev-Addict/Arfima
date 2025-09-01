use nom::{IResult, branch::alt, bytes::complete::tag, combinator::map};

pub struct QuitCommand {
    all: bool,
}

impl QuitCommand {
    pub fn new(all: bool) -> Self {
        Self { all }
    }

    pub fn all(&self) -> bool {
        self.all
    }
}

pub fn parse_quit_command(input: &str) -> IResult<&str, QuitCommand> {
    alt((
        map(alt((tag("qa"), tag("quitall"))), |_| QuitCommand::new(true)),
        map(alt((tag("q"), tag("quit"))), |_| QuitCommand::new(false)),
    ))(input)
}
