use std::str::FromStr;

use crate::input::{ParseExt, ReadLines};
use crate::{Error, Result, Solution, error};

const INPUT: &[u8] = include_bytes!("../input/day1");

pub struct Day1;

impl Solution for Day1 {
	fn part_one(&self) -> Result<String> {
		let instructions = parse_instructions(INPUT)?;
		let nb_stops_at_zero = execute_instructions_and_count_nb_stops_at_zero(&instructions);
		Ok(format!(
			"Number of times the dial stops at zero: {nb_stops_at_zero}"
		))
	}

	fn part_two(&self) -> Result<String> {
		todo!()
	}
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
	Left,
	Right,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Instruction {
	direction: Direction,
	amount: i32,
}

impl Instruction {
	fn new(direction: Direction, amount: i32) -> Self {
		Self { direction, amount }
	}
}

impl FromStr for Instruction {
	type Err = Error;

	fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
		let (d, a) = s.split_at(1);
		let amount = a.parse::<i32>()?;
		match d {
			"L" => Ok(Self::new(Direction::Left, amount)),
			"R" => Ok(Self::new(Direction::Right, amount)),
			_ => Err(error!("Invalid direction: {d}")),
		}
	}
}

fn parse_instructions(instructions: &[u8]) -> Result<Vec<Instruction>> {
	instructions.read_lines().parse::<Instruction>().collect()
}

fn execute_instructions_and_count_nb_stops_at_zero(instructions: &[Instruction]) -> usize {
	let mut dial = 50;
	let mut nb_zero = 0;
	for instruction in instructions {
		match *instruction {
			Instruction {
				direction: Direction::Left,
				amount,
			} => dial -= amount,
			Instruction {
				direction: Direction::Right,
				amount,
			} => dial += amount,
		}
		dial = dial.rem_euclid(100);
		if dial == 0 {
			nb_zero += 1;
		}
	}
	nb_zero
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::day1::Direction::{Left, Right};

	const EXAMPLE: &[u8] = b"\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

	#[test]
	fn parse_instructions_should_parse_example() {
		assert_eq!(
			parse_instructions(EXAMPLE).unwrap(),
			vec![
				Instruction::new(Left, 68),
				Instruction::new(Left, 30),
				Instruction::new(Right, 48),
				Instruction::new(Left, 5),
				Instruction::new(Right, 60),
				Instruction::new(Left, 55),
				Instruction::new(Left, 1),
				Instruction::new(Left, 99),
				Instruction::new(Right, 14),
				Instruction::new(Left, 82),
			]
		);
	}

	#[test]
	fn execute_instructions_and_count_nb_stops_at_zero_should_return_3_for_example() {
		let instructions = parse_instructions(EXAMPLE).unwrap();
		assert_eq!(
			execute_instructions_and_count_nb_stops_at_zero(&instructions),
			3,
		);
	}
}
