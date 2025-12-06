use regex::Regex;
use std::str::FromStr;
use std::sync::LazyLock;

use crate::input::ReadLines;
use crate::{Error, Result, Solution, error};

const INPUT: &[u8] = include_bytes!("../input/day6");

pub struct Day6;

impl Solution for Day6 {
	fn part_one(&self) -> Result<String> {
		let problems = parse_problems(INPUT)?;
		let sum_of_all_answers = sum_of_all_problem_answers(&problems);
		Ok(format!("Sum of all answers: {sum_of_all_answers}"))
	}

	fn part_two(&self) -> Result<String> {
		todo!()
	}
}

fn sum_of_all_problem_answers(problems: &[Problem]) -> u64 {
	problems.iter().map(Problem::evaluate).sum()
}

fn parse_problems(input: &[u8]) -> Result<Vec<Problem>> {
	static SEPARATOR: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\s+").unwrap());
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
	fn parse_example() {
		let problems = parse_problems(EXAMPLE).unwrap();

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
	fn sum_of_all_problem_answers_example() {
		let problems = parse_problems(EXAMPLE).unwrap();

		assert_eq!(sum_of_all_problem_answers(&problems), 4277556);
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
