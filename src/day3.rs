use std::str::FromStr;

use crate::input::{ParseExt, ReadLines};
use crate::{Error, Result, Solution, error};

const INPUT: &[u8] = include_bytes!("../input/day3");

pub struct Day3;

impl Solution for Day3 {
	fn part_one(&self) -> Result<String> {
		let banks = parse_banks(INPUT)?;
		let sum = sum_of_largest_joltages(&banks);
		Ok(format!("Sum of largest joltages: {sum}"))
	}

	fn part_two(&self) -> Result<String> {
		todo!()
	}
}

fn sum_of_largest_joltages(banks: &[Bank]) -> u32 {
	banks.iter().map(Bank::largest_joltage).sum()
}

struct Bank(Vec<u32>);

impl Bank {
	fn largest_joltage(&self) -> u32 {
		let mut largest_joltage = 0;
		for i in 0..(self.0.len() - 1) {
			for end in &self.0[i + 1..] {
				let joltage = self.0[i] * 10 + end;
				largest_joltage = largest_joltage.max(joltage);
			}
		}
		largest_joltage
	}
}

impl FromStr for Bank {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self> {
		s.chars()
			.map(|c| c.to_digit(10).ok_or_else(|| error!("Invalid digit: {c}")))
			.collect::<Result<_>>()
			.map(Self)
	}
}

fn parse_banks(input: &[u8]) -> Result<Vec<Bank>> {
	input.read_lines().parse().collect()
}

#[cfg(test)]
mod test {
	use super::*;

	const EXAMPLE: &[u8] = b"\
987654321111111
811111111111119
234234234234278
818181911112111
";

	#[test]
	fn bank_largest_voltage_should_return_98_for_987654321111111() {
		let bank: Bank = "987654321111111".parse().unwrap();
		assert_eq!(bank.largest_joltage(), 98);
	}

	#[test]
	fn bank_largest_voltage_should_return_89_for_811111111111119() {
		let bank: Bank = "811111111111119".parse().unwrap();
		assert_eq!(bank.largest_joltage(), 89);
	}

	#[test]
	fn sum_of_largest_joltages_should_return_357_for_example() {
		let example_banks = parse_banks(EXAMPLE).unwrap();
		assert_eq!(sum_of_largest_joltages(&example_banks), 357);
	}
}
