use std::ops::RangeInclusive;

use crate::{Result, Solution, error};

const INPUT: &str = include_str!("../input/day2");

pub struct Day2;

impl Solution for Day2 {
	fn part_one(&self) -> crate::Result<String> {
		let instruction_ranges = parse_instruction_ranges(INPUT)?;
		let sum_of_invalid_ids: u64 = find_invalid_ids(&instruction_ranges).sum();
		Ok(format!("Sum of invalid ids: {sum_of_invalid_ids}"))
	}

	fn part_two(&self) -> crate::Result<String> {
		todo!()
	}
}

fn find_invalid_ids(id_ranges: &[RangeInclusive<u64>]) -> impl Iterator<Item = u64> {
	id_ranges
		.iter()
		.flat_map(|range| range.clone())
		.filter(|id| !is_valid_id(*id))
}

fn parse_instruction_ranges(input: &str) -> Result<Vec<RangeInclusive<u64>>> {
	input.split(',').map(parse_instruction_range).collect()
}

fn parse_instruction_range(range: &str) -> Result<RangeInclusive<u64>> {
	let (start, end) = range
		.split_once('-')
		.ok_or(error!("Invalid instruction range: {range}"))?;
	Ok(start.parse()?..=end.parse()?)
}

fn is_valid_id(id: u64) -> bool {
	let id_as_string = id.to_string();
	let digits = id_as_string.as_bytes();
	let id_length = digits.len();
	if !id_length.is_multiple_of(2) {
		return true;
	}
	for i in 0..(id_length / 2) {
		if digits[i] != digits[i + id_length / 2] {
			return true;
		}
	}
	false
}

#[cfg(test)]
mod test {
	use super::*;

	const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

	#[test]
	fn parse_instruction_ranges_should_parse_example() {
		let result = parse_instruction_ranges(EXAMPLE);

		assert_eq!(
			result.unwrap(),
			vec![
				11..=22,
				95..=115,
				998..=1012,
				1188511880..=1188511890,
				222220..=222224,
				1698522..=1698528,
				446443..=446449,
				38593856..=38593862,
				565653..=565659,
				824824821..=824824827,
				2121212118..=2121212124
			]
		);
	}

	#[test]
	fn is_valid_id_should_return_true_for_1() {
		assert!(is_valid_id(1))
	}

	#[test]
	fn is_valid_id_should_return_false_for_11() {
		assert!(!is_valid_id(11))
	}

	#[test]
	fn is_valid_id_should_return_true_for_101() {
		assert!(is_valid_id(101))
	}

	#[test]
	fn is_valid_id_should_return_false_for_6464() {
		assert!(!is_valid_id(6464))
	}

	#[test]
	fn is_valid_id_should_return_false_for_123123() {
		assert!(!is_valid_id(123123))
	}

	#[test]
	fn sum_of_invalid_ids_for_example_should_be_1227775554() {
		let instruction_ranges = parse_instruction_ranges(EXAMPLE).unwrap();
		assert_eq!(
			find_invalid_ids(&instruction_ranges).sum::<u64>(),
			1227775554,
		);
	}
}
