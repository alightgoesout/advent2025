use regex::Regex;
use std::str::FromStr;
use std::sync::LazyLock;

use crate::input::ReadLines;
use crate::{Error, Result, Solution, error};

const INPUT: &[u8] = include_bytes!("../input/day6");

pub struct Day6;

impl Solution for Day6 {
	fn part_one(&self) -> Result<String> {
		let problems = parse_problems_part1(INPUT)?;
		let sum_of_all_answers = sum_of_all_problem_answers(&problems);
		Ok(format!("Sum of all answers: {sum_of_all_answers}"))
	}

	fn part_two(&self) -> Result<String> {
		let problems = parse_problems_part2(INPUT)?;
		let sum_of_all_answers = sum_of_all_problem_answers(&problems);
		Ok(format!("Sum of all answers: {sum_of_all_answers}"))
	}
}

fn sum_of_all_problem_answers(problems: &[Problem]) -> u64 {
	problems.iter().map(Problem::evaluate).sum()
}

static SEPARATOR: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\s+").unwrap());

fn parse_problems_part1(input: &[u8]) -> Result<Vec<Problem>> {
	let lines: Vec<_> = input.read_lines().collect();
	let (operators, operand_lines) = lines.split_last().ok_or_else(|| error!("Empty input"))?;

	let operand_lines = operand_lines
		.iter()
		.map(|line| {
			SEPARATOR
				.split(line.trim())
				.map(|operand| operand.parse::<u64>().map_err(Error::from))
				.collect::<Result<Vec<_>>>()
		})
		.collect::<Result<Vec<_>>>()?;

	let operators = SEPARATOR
		.split(operators)
		.map(|operator| operator.parse::<Operator>())
		.collect::<Result<Vec<_>>>()?;

	Ok(operators
		.into_iter()
		.enumerate()
		.map(|(i, operator)| Problem {
			operator,
			operands: operand_lines.iter().map(|line| line[i]).collect(),
		})
		.collect())
}

fn parse_problems_part2(input: &[u8]) -> Result<Vec<Problem>> {
	let lines: Vec<_> = input.read_lines().collect();
	let (operators, operand_lines) = lines.split_last().ok_or_else(|| error!("Empty input"))?;

	let operators = SEPARATOR
		.split(operators)
		.map(|operator| operator.parse::<Operator>())
		.collect::<Result<Vec<_>>>()?;

	let mut index = 0;
	operators
		.into_iter()
		.map(|operator| {
			let operands = parse_operands(operand_lines, &mut index)?;
			Ok(Problem { operator, operands })
		})
		.collect()
}

fn parse_operands(operand_lines: &[String], index: &mut usize) -> Result<Vec<u64>> {
	let mut operands = Vec::new();

	while *index < operand_lines[0].len() {
		let digits: String = operand_lines
			.iter()
			.map(|line| line.as_bytes()[*index] as char)
			.collect();
		*index += 1;
		let operand = digits.trim();
		if operand.is_empty() {
			break;
		}
		operands.push(operand.parse()?);
	}

	Ok(operands)
}

#[derive(Debug, Eq, PartialEq)]
enum Operator {
	Add,
	Multiply,
}

impl FromStr for Operator {
	type Err = Error;

	fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
		match s {
			"+" => Ok(Self::Add),
			"*" => Ok(Self::Multiply),
			_ => Err(error!("Invalid operator: {s}")),
		}
	}
}

#[derive(Debug, Eq, PartialEq)]
struct Problem {
	operator: Operator,
	operands: Vec<u64>,
}

impl Problem {
	fn evaluate(&self) -> u64 {
		match self.operator {
			Operator::Add => self.operands.iter().sum(),
			Operator::Multiply => self.operands.iter().product(),
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	const EXAMPLE: &[u8] = b"\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +
";

	#[test]
	fn parse_example_part1() {
		let problems = parse_problems_part1(EXAMPLE).unwrap();

		assert_eq!(
			problems,
			vec![
				multiply!(123, 45, 6),
				add!(328, 64, 98),
				multiply!(51, 387, 215),
				add!(64, 23, 314),
			],
		);
	}

	#[test]
	fn sum_of_all_problem_answers_example_part1() {
		let problems = parse_problems_part1(EXAMPLE).unwrap();

		assert_eq!(sum_of_all_problem_answers(&problems), 4277556);
	}

	#[test]
	fn parse_example_part2() {
		let problems = parse_problems_part2(EXAMPLE).unwrap();

		assert_eq!(
			problems,
			vec![
				multiply!(1, 24, 356),
				add!(369, 248, 8),
				multiply!(32, 581, 175),
				add!(623, 431, 4),
			],
		);
	}

	#[test]
	fn sum_of_all_problem_answers_example_part2() {
		let problems = parse_problems_part2(EXAMPLE).unwrap();

		assert_eq!(sum_of_all_problem_answers(&problems), 3263827);
	}

	macro_rules! add {
    	($($x:expr),+$(,)?) => {
			Problem {
				operator: Operator::Add,
				operands: vec![$($x),+]
			}
		};
	}

	macro_rules! multiply {
    	($($x:expr),+$(,)?) => {
			Problem {
				operator: Operator::Multiply,
				operands: vec![$($x),+]
			}
		};
	}

	pub(crate) use add;
	pub(crate) use multiply;
}
