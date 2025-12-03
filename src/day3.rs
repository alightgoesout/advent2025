use std::cell::OnceCell;
use std::str::FromStr;

use crate::input::{ParseExt, ReadLines};
use crate::{Error, Result, Solution, error};

const INPUT: &[u8] = include_bytes!("../input/day3");

#[derive(Default)]
pub struct Day3(OnceCell<Vec<Bank>>);

impl Day3 {
	fn banks(&self) -> Result<&Vec<Bank>> {
		self.0.get_or_try_init(|| parse_banks(INPUT))
	}
}

impl Solution for Day3 {
	fn part_one(&self) -> Result<String> {
		let sum = sum_of_largest_joltages(self.banks()?, 2);
		Ok(format!("Sum of largest joltages: {sum}"))
	}

	fn part_two(&self) -> Result<String> {
		let sum = sum_of_largest_joltages(self.banks()?, 12);
		Ok(format!(
			"Sum of largest joltages with safety override: {sum}"
		))
	}
}

fn sum_of_largest_joltages(banks: &[Bank], nb_batteries: usize) -> u64 {
	banks
		.iter()
		.map(|bank| bank.largest_joltage(nb_batteries))
		.sum()
}

struct Bank(Vec<u64>);

impl Bank {
	fn largest_joltage(&self, nb_batteries: usize) -> u64 {
		let mut batteries = Vec::new();
		let mut start = 0;
		while batteries.len() < nb_batteries {
			let mut battery = start;
			for i in (start + 1)..=(self.0.len() + batteries.len() - nb_batteries) {
				if self.0[i] > self.0[battery] {
					battery = i;
				}
			}
			batteries.push(battery);
			start = battery + 1;
		}
		self.joltage(&batteries)
	}

	fn joltage(&self, batteries: &[usize]) -> u64 {
		let mut joltage = 0;
		for (i, battery) in batteries.iter().enumerate() {
			joltage += self.0[*battery] * 10u64.pow((batteries.len() - i - 1) as u32);
		}
		joltage
	}
}

impl FromStr for Bank {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self> {
		s.chars()
			.map(|c| {
				c.to_digit(10)
					.map(|d| d as u64)
					.ok_or_else(|| error!("Invalid digit: {c}"))
			})
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
	fn bank_largest_joltage_should_return_98_for_987654321111111_and_2_batteries() {
		let bank: Bank = "987654321111111".parse().unwrap();
		assert_eq!(bank.largest_joltage(2), 98);
	}

	#[test]
	fn bank_largest_joltage_should_return_89_for_811111111111119_and_2_batteries() {
		let bank: Bank = "811111111111119".parse().unwrap();
		assert_eq!(bank.largest_joltage(2), 89);
	}

	#[test]
	fn bank_largest_joltage_should_return_92_for_818181911112111_and_2_batteries() {
		let bank: Bank = "818181911112111".parse().unwrap();
		assert_eq!(bank.largest_joltage(2), 92);
	}

	#[test]
	fn sum_of_largest_joltages_should_return_357_for_example_and_2_batteries() {
		let example_banks = parse_banks(EXAMPLE).unwrap();
		assert_eq!(sum_of_largest_joltages(&example_banks, 2), 357);
	}

	#[test]
	fn bank_largest_joltage_should_return_987654321111_for_987654321111111_and_12_batteries() {
		let bank: Bank = "987654321111111".parse().unwrap();
		assert_eq!(bank.largest_joltage(12), 987654321111);
	}

	#[test]
	fn sum_of_largest_joltages_should_return_3121910778619_for_example_and_12_batteries() {
		let example_banks = parse_banks(EXAMPLE).unwrap();
		assert_eq!(sum_of_largest_joltages(&example_banks, 12), 3121910778619);
	}
}
