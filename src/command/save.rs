use nom::{IResult, branch::alt, bytes::complete::tag, combinator::map};

pub fn parse_save_command(input: &str) -> IResult<&str, ()> {
    map(alt((tag("s"), tag("save"))), |_| ())(input)
}
