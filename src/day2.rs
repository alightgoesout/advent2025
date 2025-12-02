use std::cell::OnceCell;
use std::ops::RangeInclusive;

use crate::{Result, Solution, error};

const INPUT: &str = include_str!("../input/day2");

#[derive(Default)]
pub struct Day2(OnceCell<Vec<RangeInclusive<u64>>>);

impl Day2 {
	fn instruction_ranges(&self) -> Result<&Vec<RangeInclusive<u64>>> {
		self.0.get_or_try_init(|| parse_instruction_ranges(INPUT))
	}
}

impl Solution for Day2 {
	fn part_one(&self) -> Result<String> {
		let sum_of_invalid_ids: u64 = find_invalid_ids_part1(self.instruction_ranges()?).sum();
		Ok(format!("Sum of invalid ids: {sum_of_invalid_ids}"))
	}

	fn part_two(&self) -> Result<String> {
		let sum_of_invalid_ids: u64 = find_invalid_ids_part2(self.instruction_ranges()?).sum();
		Ok(format!("Sum of invalid ids: {sum_of_invalid_ids}"))
	}
}

fn find_invalid_ids_part1(id_ranges: &[RangeInclusive<u64>]) -> impl Iterator<Item = u64> {
	id_ranges
		.iter()
		.flat_map(|range| range.clone())
		.filter(|id| !is_valid_id_part1(*id))
}

fn find_invalid_ids_part2(id_ranges: &[RangeInclusive<u64>]) -> impl Iterator<Item = u64> {
	id_ranges
		.iter()
		.flat_map(|range| range.clone())
		.filter(|id| !is_valid_id_part2(*id))
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

fn is_valid_id_part1(id: u64) -> bool {
	let id_as_string = id.to_string();
	let digits = id_as_string.as_bytes();
	!digits.len().is_multiple_of(2) || !has_repeated_pattern(digits, digits.len() / 2)
}

fn is_valid_id_part2(id: u64) -> bool {
	let id_as_string = id.to_string();
	let digits = id_as_string.as_bytes();
	for size in 1..=(digits.len() - 1) {
		if has_repeated_pattern(digits, size) {
			return false;
		}
	}
	true
}

fn has_repeated_pattern(digits: &[u8], size: usize) -> bool {
	if !digits.len().is_multiple_of(size) {
		return false;
	}
	for i in 0..size {
		for j in 1..(digits.len() / size) {
			if digits[i] != digits[i + j * size] {
				return false;
			}
		}
	}
	true
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
	fn is_valid_id_part1_should_return_true_for_1() {
		assert!(is_valid_id_part1(1))
	}

	#[test]
	fn is_valid_id_part1_should_return_false_for_11() {
		assert!(!is_valid_id_part1(11))
	}

	#[test]
	fn is_valid_id_part1_should_return_true_for_101() {
		assert!(is_valid_id_part1(101))
	}

	#[test]
	fn is_valid_id_part1_should_return_true_for_111() {
		assert!(is_valid_id_part1(111))
	}

	#[test]
	fn is_valid_id_part1_should_return_true_for_1011() {
		assert!(is_valid_id_part1(1011))
	}

	#[test]
	fn is_valid_id_part1_should_return_true_for_1110() {
		assert!(is_valid_id_part1(1110))
	}

	#[test]
	fn is_valid_id_part1_should_return_false_for_6464() {
		assert!(!is_valid_id_part1(6464))
	}

	#[test]
	fn is_valid_id_part1_should_return_false_for_123123() {
		assert!(!is_valid_id_part1(123123))
	}

	#[test]
	fn sum_of_invalid_ids_part1_for_example_should_be_1227775554() {
		let instruction_ranges = parse_instruction_ranges(EXAMPLE).unwrap();
		assert_eq!(
			find_invalid_ids_part1(&instruction_ranges).sum::<u64>(),
			1227775554,
		);
	}

	#[test]
	fn is_valid_id_part2_should_return_true_for_1() {
		assert!(is_valid_id_part2(1))
	}

	#[test]
	fn is_valid_id_part2_should_return_false_for_11() {
		assert!(!is_valid_id_part2(11))
	}

	#[test]
	fn is_valid_id_part2_should_return_true_for_101() {
		assert!(is_valid_id_part2(101))
	}

	#[test]
	fn is_valid_id_part2_should_return_false_for_6464() {
		assert!(!is_valid_id_part2(6464))
	}

	#[test]
	fn is_valid_id_part2_should_return_false_for_123123123() {
		assert!(!is_valid_id_part2(123123123))
	}

	#[test]
	fn is_valid_id_part2_should_return_false_for_1212121212() {
		assert!(!is_valid_id_part2(1212121212))
	}

	#[test]
	fn is_valid_id_part2_should_return_false_for_1111111() {
		assert!(!is_valid_id_part2(1111111))
	}

	#[test]
	fn sum_of_invalid_ids_part2_for_example_should_be_4174379265() {
		let instruction_ranges = parse_instruction_ranges(EXAMPLE).unwrap();
		assert_eq!(
			find_invalid_ids_part2(&instruction_ranges).sum::<u64>(),
			4174379265,
		);
	}
}
