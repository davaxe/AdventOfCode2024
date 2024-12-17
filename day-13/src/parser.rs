use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{self, anychar, line_ending};
use nom::multi::{many1, separated_list1};
use nom::sequence::{preceded, separated_pair};
use nom::IResult;

#[derive(Debug, PartialEq)]
pub(crate) struct ClawMachine {
    /// Change in position for button A
    pub button_a: (i32, i32),
    /// Change in position for button B
    pub button_b: (i32, i32),
    pub price_location: (i32, i32),
}

fn number(input: &str) -> IResult<&str, i32> {
    let (input, _) = anychar(input)?;
    let (input, sign) = anychar(input)?;
    let (input, number) = complete::i32(input)?;
    Ok((input, if sign == '-' { -number } else { number }))
}

fn button(input: &str) -> IResult<&str, (i32, i32)> {
    let (input, _) = alt((tag("Button A: "), tag("Button B: ")))(input)?;
    let (input, (dx, dy)) = separated_pair(number, tag(", "), number)(input)?;
    Ok((input, (dx, dy)))
}

fn prize(input: &str) -> IResult<&str, (i32, i32)> {
    let (input, _) = tag("Prize: ")(input)?;
    let (input, (dx, dy)) = separated_pair(
        preceded(tag("X="), complete::i32),
        tag(", "),
        preceded(tag("Y="), complete::i32),
    )(input)?;

    Ok((input, (dx, dy)))
}

fn claw_machine(input: &str) -> IResult<&str, ClawMachine> {
    let (input, button_a) = button(input)?;
    let (input, _) = line_ending(input)?;
    let (input, button_b) = button(input)?;
    let (input, _) = line_ending(input)?;
    let (input, price_location) = prize(input)?;
    Ok((
        input,
        ClawMachine {
            button_a,
            button_b,
            price_location,
        },
    ))
}

pub(crate) fn claw_machines(input: &str) -> IResult<&str, Vec<ClawMachine>> {
    separated_list1(many1(line_ending), claw_machine)(input)
}
