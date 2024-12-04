use nom::branch::alt;
use nom::bytes::complete::{tag, take_till};
use nom::character::complete;
use nom::multi::many0;
use nom::sequence::{delimited, preceded, separated_pair};
use nom::IResult;

// ---- Part 1 -----------------------------------------------------------------
type Multiplication = (i32, i32);

fn multiplication(input: &str) -> IResult<&str, Multiplication> {
    preceded(
        tag("mul"),
        delimited(
            tag("("),
            separated_pair(complete::i32, tag(","), complete::i32),
            tag(")"),
        ),
    )(input)
}

fn find_multiplications(input: &str) -> IResult<&str, Option<Multiplication>> {
    let (input, _) = take_till(|c| c == 'm')(input)?;
    match multiplication(input) {
        Ok((input, multiplication)) => Ok((input, Some(multiplication))),
        Err(_) if !input.is_empty() => Ok((&input[1..], None)),
        Err(e) => Err(e),
    }
}

pub fn multiplications(input: &str) -> IResult<&str, Vec<Multiplication>> {
    let (_, multiplications) = many0(find_multiplications)(input)?;
    Ok(("", multiplications.into_iter().flatten().collect()))
}

// ---- Part 2 -----------------------------------------------------------------
pub enum Operation {
    /// Disable future multiplications
    Disable,
    /// Enable future multiplications
    Enable,
    /// Perform a multiplication
    Multiplication(Multiplication),
}

fn enable(input: &str) -> IResult<&str, Operation> {
    tag("do()")(input).map(|(input, _)| (input, Operation::Enable))
}

fn disable(input: &str) -> IResult<&str, Operation> {
    tag("don't()")(input).map(|(input, _)| (input, Operation::Disable))
}

fn multiplication_operation(input: &str) -> IResult<&str, Operation> {
    multiplication(input).map(|(input, multiplication)| {
        (input, Operation::Multiplication(multiplication))
    })
}

fn find_operation(input: &str) -> IResult<&str, Option<Operation>> {
    let (input, _) = take_till(|c| c == 'd' || c == 'm')(input)?;
    match alt((enable, disable, multiplication_operation))(input) {
        Ok((input, operation)) => Ok((input, Some(operation))),
        Err(_) if !input.is_empty() => Ok((&input[1..], None)),
        Err(e) => Err(e),
    }
}

pub fn operations(input: &str) -> IResult<&str, Vec<Operation>> {
    let (_, operations) = many0(find_operation)(input)?;
    Ok(("", operations.into_iter().flatten().collect()))
}
