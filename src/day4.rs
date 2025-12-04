use std::cell::OnceCell;
use std::str::FromStr;

use crate::{Error, Result, Solution};

const INPUT: &str = include_str!("../input/day4");

#[derive(Default)]
pub struct Day4(OnceCell<Warehouse<136>>);

impl Day4 {
	fn warehouse(&self) -> Result<&Warehouse<136>> {
		self.0.get_or_try_init(|| INPUT.parse())
	}
}

impl Solution for Day4 {
	fn part_one(&self) -> Result<String> {
		let nb_accessible_rolls = self.warehouse()?.count_accessible_rolls();
		Ok(format!("Number of accessible rolls: {nb_accessible_rolls}"))
	}

	fn part_two(&self) -> Result<String> {
		let (_, removed) = remove_all_rolls(self.warehouse()?);
		Ok(format!("Number of removed rolls: {removed}"))
	}
}

fn remove_all_rolls<const N: usize>(warehouse: &Warehouse<N>) -> (Warehouse<N>, usize) {
	let mut current = *warehouse;
	let mut total_removed = 0;
	loop {
		let (new_warehouse, removed) = current.remove_rolls();
		if removed == 0 {
			break;
		}
		current = new_warehouse;
		total_removed += removed;
	}
	(current, total_removed)
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
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

	fn remove_rolls(&self) -> (Self, usize) {
		let mut spaces = [[false; N]; N];
		let mut removed = 0;
		for (r, row) in self.0.iter().enumerate() {
			for (c, has_roll) in row.iter().enumerate() {
				if *has_roll {
					if self.count_adjacent_rolls(c, r) >= 4 {
						spaces[r][c] = true;
					} else {
						removed += 1;
					}
				}
			}
		}
		(Self(spaces), removed)
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

	#[test]
	fn remove_rolls_example() {
		let warehouse: Warehouse<10> = EXAMPLE.parse().unwrap();
		let (new_warehouse, removed) = warehouse.remove_rolls();
		assert_eq!(removed, 13);
		assert_eq!(
			new_warehouse,
			"\
.......@..
.@@.@.@.@@
@@@@@...@@
@.@@@@..@.
.@.@@@@.@.
.@@@@@@@.@
.@.@.@.@@@
..@@@.@@@@
.@@@@@@@@.
....@@@...
"
			.parse::<Warehouse<10>>()
			.unwrap()
		);
	}

	#[test]
	fn remove_all_rolls_example() {
		let warehouse: Warehouse<10> = EXAMPLE.parse().unwrap();
		let (new_warehouse, removed) = remove_all_rolls(&warehouse);
		assert_eq!(removed, 43);
		assert_eq!(
			new_warehouse,
			"\
..........
..........
..........
...x@@....
...@@@@...
...@@@@@..
...@.@.@@.
...@@.@@@.
...@@@@@..
....@@@...
"
			.parse::<Warehouse<10>>()
			.unwrap()
		);
	}
}
