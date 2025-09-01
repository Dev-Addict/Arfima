use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, multispace1},
    combinator::{map, opt},
};

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

pub fn parse_set_command(input: &str) -> IResult<&str, SetCommand> {
    let (input, _) = alt((tag("set"), tag("se")))(input)?;
    let (input, _) = multispace1(input)?;

    alt((
        map(parse_history_size_assignment, SetCommand::Set),
        parse_boolean_set_command,
    ))(input)
}
