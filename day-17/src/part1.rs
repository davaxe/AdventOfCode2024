use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{self, anychar, line_ending};
use nom::multi::{many0, separated_list1};
use nom::sequence::preceded;
use nom::IResult;

#[derive(Debug)]
enum Operand {
    Literal(u32),
    Combo(u32),
}

#[derive(Debug)]
enum Instruction {
    /// Divide value in A register by Combo Operand raised to the second power
    Adv(Operand),
    /// bitwise xor of register B and a literal operand
    Bxl(Operand),
    /// Combo operand modulo 8 -> write to B register
    Bst(Operand),
    /// Does nothing if A register is 0, otherwise jumps to the instruction
    /// pointer to the Operand value
    Jnz(Operand),
    /// Calculates bitwise xor of register B and register C, storing the result
    /// in register B (Reads the operand but does not use it)
    Bxc,
    /// Calculates the value of its combo operand modulo 8, and outputs the
    /// results (comma separated if multiple)
    Out(Operand),
    /// Same as adv, but stores the result in register B
    Bdv(Operand),
    /// Same as adv, but stores the result in register C
    Cdv(Operand),
}

impl Instruction {
    fn from_opcode(opcode: u32, operand: u32) -> Option<Self> {
        match opcode {
            0 => Some(Self::Adv(Operand::Combo(operand))),
            1 => Some(Self::Bxl(Operand::Literal(operand))),
            2 => Some(Self::Bst(Operand::Combo(operand))),
            3 => Some(Self::Jnz(Operand::Literal(operand))),
            4 => Some(Self::Bxc),
            5 => Some(Self::Out(Operand::Combo(operand))),
            6 => Some(Self::Bdv(Operand::Combo(operand))),
            7 => Some(Self::Cdv(Operand::Combo(operand))),
            _ => None,
        }
    }
}

enum InstructionResult {
    Continue,
    Output(u32),
    Done,
}

#[derive(Debug)]
struct Computer {
    instruction_pointer: usize,
    registers: [u32; 3],
    instructions: Vec<Instruction>,
}

impl Computer {
    fn run(self) -> Vec<u32> {
        let mut output = Vec::new();
        let mut computer = self;

        while match computer.step() {
            InstructionResult::Continue => true,
            InstructionResult::Output(value) => {
                output.push(value);
                true
            }
            InstructionResult::Done => false,
        } {}

        output
    }

    fn step(&mut self) -> InstructionResult {
        if self.instruction_pointer >= self.instructions.len() {
            return InstructionResult::Done;
        }

        let result = match &self.instructions[self.instruction_pointer] {
            Instruction::Adv(operand) => {
                self.registers[0] >>= self.get_value(operand);
                InstructionResult::Continue
            }
            Instruction::Bxl(operand) => {
                self.registers[1] ^= self.get_value(operand);
                InstructionResult::Continue
            }
            Instruction::Bst(operand) => {
                self.registers[1] = self.get_value(operand) & 7;
                InstructionResult::Continue
            }
            Instruction::Jnz(operand) => {
                if self.registers[0] == 0 {
                    InstructionResult::Continue
                } else {
                    self.instruction_pointer = self.get_value(operand) as usize;
                    return InstructionResult::Continue;
                }
            }
            Instruction::Bxc => {
                self.registers[1] ^= self.registers[2];
                InstructionResult::Continue
            }
            Instruction::Out(operand) => {
                InstructionResult::Output(self.get_value(operand) & 7)
            }
            Instruction::Bdv(operand) => {
                self.registers[1] =
                    self.registers[0] >> self.get_value(operand);
                InstructionResult::Continue
            }
            Instruction::Cdv(operand) => {
                self.registers[2] =
                    self.registers[0] >> self.get_value(operand);
                InstructionResult::Continue
            }
        };
        self.instruction_pointer += 1;
        result
    }

    fn get_value(&self, operand: &Operand) -> u32 {
        match operand {
            Operand::Literal(val) | Operand::Combo(val @ 0..=3) => *val,
            Operand::Combo(val @ 4..=6) => self.registers[(*val - 4) as usize],
            _ => panic!("Invalid!"),
        }
    }
}

#[must_use]
pub fn task(input: &str) -> Option<String> {
    let (_, computer) = computer(input).ok()?;
    println!("{:?}", computer);
    let output = computer.run();

    Some(output.iter().join(","))
}

fn register(input: &str) -> IResult<&str, (char, u32)> {
    let (input, _) = tag("Register ")(input)?;
    let (input, register) = anychar(input)?;
    let (input, value) = preceded(tag(": "), complete::u32)(input)?;
    Ok((input, (register, value)))
}

fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, _) = tag("Program: ")(input)?;
    let (input, instructions) =
        separated_list1(tag(","), complete::u32)(input)?;

    let instructions = instructions
        .chunks(2)
        .map(|window| {
            let opcode = window[0];
            let operand = window[1];
            Instruction::from_opcode(opcode, operand).unwrap()
        })
        .collect();

    Ok((input, instructions))
}

fn computer(input: &str) -> IResult<&str, Computer> {
    let (input, (_, val1)) = register(input)?;
    let (input, _) = line_ending(input)?;
    let (input, (_, val2)) = register(input)?;
    let (input, _) = line_ending(input)?;
    let (input, (_, val3)) = register(input)?;
    let (input, _) = many0(line_ending)(input)?;
    let (input, instructions) = instructions(input)?;

    Ok((
        input,
        Computer {
            instruction_pointer: 0,
            registers: [val1, val2, val3],
            instructions,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn alt_test_task() {
        let input = "Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4
";
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "0,1,2");
    }
}
