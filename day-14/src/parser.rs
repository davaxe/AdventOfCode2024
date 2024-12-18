use nom::bytes::complete::tag;
use nom::character::complete::{self, line_ending};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;

#[derive(Debug)]
pub struct Robot {
    pub position: (i32, i32),
    pub velocity: (i32, i32),
}

impl Robot {
    pub fn step(&mut self, width: i32, height: i32) {
        let (x, y) = self.position;
        let (dx, dy) = self.velocity;

        // Move the robot, teleport it to the other side of the screen if it
        // goes out of bounds.
        self.position = ((x + dx + width) % width, (y + dy + height) % height);
    }
}

fn vec2(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(complete::i32, tag(","), complete::i32)(input)
}

fn robot(input: &str) -> IResult<&str, Robot> {
    let (input, _) = tag("p=")(input)?;
    let (input, position) = vec2(input)?;
    let (input, _) = tag(" v=")(input)?;
    let (input, velocity) = vec2(input)?;

    Ok((input, Robot { position, velocity }))
}

pub fn robots(input: &str) -> IResult<&str, Vec<Robot>> {
    separated_list1(line_ending, robot)(input)
}
