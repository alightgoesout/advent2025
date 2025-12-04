use std::str::FromStr;

use crate::{Error, Result, Solution};

const INPUT: &str = include_str!("../input/day4");

pub struct Day4;

impl Solution for Day4 {
	fn part_one(&self) -> Result<String> {
		let warehouse: Warehouse<136> = INPUT.parse()?;
		let nb_accessible_rolls = warehouse.count_accessible_rolls();
		Ok(format!("Number of accessible rolls: {nb_accessible_rolls}"))
	}

	fn part_two(&self) -> Result<String> {
		todo!()
	}
}

struct Warehouse<const N: usize>([[bool; N]; N]);

impl<const N: usize> Warehouse<N> {
	fn count_adjacent_rolls(&self, column: usize, row: usize) -> usize {
		let mut rolls = 0;
		if row > 0 {
			let previous_row = self.0[row - 1];
			if column > 0 && previous_row[column - 1] {
				rolls += 1;
			}
			if previous_row[column] {
				rolls += 1;
			}
			if column < N - 1 && previous_row[column + 1] {
				rolls += 1;
			}
		}
		if column > 0 && self.0[row][column - 1] {
			rolls += 1;
		}
		if column < N - 1 && self.0[row][column + 1] {
			rolls += 1;
		}
		if row < N - 1 {
			let previous_row = self.0[row + 1];
			if column > 0 && previous_row[column - 1] {
				rolls += 1;
			}
			if previous_row[column] {
				rolls += 1;
			}
			if column < N - 1 && previous_row[column + 1] {
				rolls += 1;
			}
		}
		rolls
	}

	fn count_accessible_rolls(&self) -> usize {
		let mut accessible_rows = 0;
		for (r, row) in self.0.iter().enumerate() {
			for (c, &has_roll) in row.iter().enumerate() {
				if has_roll && self.count_adjacent_rolls(c, r) < 4 {
					accessible_rows += 1;
				}
			}
		}
		accessible_rows
	}
}

impl<const N: usize> FromStr for Warehouse<N> {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self> {
		let mut spaces = [[false; N]; N];
		for (i, c) in s.chars().enumerate() {
			if i % (N + 1) == N {
				continue;
			}
			let row = i / (N + 1);
			let column = i % (N + 1);
			spaces[row][column] = c == '@';
		}
		Ok(Self(spaces))
	}
}

#[cfg(test)]
mod test {
	use super::*;

	const EXAMPLE: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

	#[test]
	fn parse_example() {
		let warehouse: Warehouse<10> = EXAMPLE.parse().unwrap();
		assert_eq!(
			warehouse.0,
			[
				[
					false, false, true, true, false, true, true, true, true, false
				],
				[
					true, true, true, false, true, false, true, false, true, true
				],
				[true, true, true, true, true, false, true, false, true, true],
				[
					true, false, true, true, true, true, false, false, true, false
				],
				[true, true, false, true, true, true, true, false, true, true],
				[false, true, true, true, true, true, true, true, false, true],
				[
					false, true, false, true, false, true, false, true, true, true
				],
				[true, false, true, true, true, false, true, true, true, true],
				[false, true, true, true, true, true, true, true, true, false],
				[
					true, false, true, false, true, true, true, false, true, false
				],
			]
		);
	}

	#[test]
	fn count_adjacent_rolls_should_return_2_for_example_0_0() {
		let warehouse: Warehouse<10> = EXAMPLE.parse().unwrap();
		assert_eq!(warehouse.count_adjacent_rolls(0, 0), 2);
	}

	#[test]
	fn count_accessible_rolls_should_return_13_for_example() {
		let warehouse: Warehouse<10> = EXAMPLE.parse().unwrap();
		assert_eq!(warehouse.count_accessible_rolls(), 13);
	}
}
