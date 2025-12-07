use std::collections::HashSet;
use std::str::FromStr;

use crate::{Error, Result, Solution};

const INPUT: &str = include_str!("../input/day7");

pub struct Day7;

impl Solution for Day7 {
	fn part_one(&self) -> Result<String> {
		let diagram: Diagram<141, 142> = INPUT.parse()?;
		let splits = diagram.count_splits();
		Ok(format!("Number of splits: {splits}"))
	}

	fn part_two(&self) -> Result<String> {
		todo!()
	}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Space {
	Empty,
	Splitter,
}

#[derive(Debug, Eq, PartialEq)]
struct Diagram<const M: usize, const N: usize> {
	start: usize,
	spaces: [[Space; M]; N],
}

impl<const M: usize, const N: usize> Diagram<M, N> {
	fn count_splits(&self) -> usize {
		let mut splits = 0;
		let mut beams = HashSet::from([self.start]);

		for row in 0..(M - 1) {
			let mut new_beams = HashSet::new();
			for beam in beams {
				if self.spaces[row + 1][beam] == Space::Splitter {
					new_beams.insert(beam - 1);
					new_beams.insert(beam + 1);
					splits += 1;
				} else {
					new_beams.insert(beam);
				}
			}
			beams = new_beams;
		}

		splits
	}
}

impl<const M: usize, const N: usize> FromStr for Diagram<M, N> {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self> {
		let mut start = 0;
		let mut spaces = [[Space::Empty; M]; N];
		for (i, c) in s.chars().enumerate() {
			if i % (M + 1) == M {
				continue;
			}
			if c == '^' {
				let row = i / (M + 1);
				let column = i % (M + 1);
				spaces[row][column] = Space::Splitter;
			} else if c == 'S' {
				start = i;
			}
		}
		Ok(Diagram { start, spaces })
	}
}

#[cfg(test)]
mod test {
	use super::{
		Space::{Empty, Splitter},
		*,
	};

	const EXAMPLE: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

	#[test]
	fn diagram_parse_example() {
		let diagram: Diagram<15, 16> = EXAMPLE.parse().unwrap();
		assert_eq!(
			diagram,
			Diagram {
				start: 7,
				spaces: [
					[
						Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
						Empty, Empty, Empty, Empty, Empty
					],
					[
						Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
						Empty, Empty, Empty, Empty, Empty
					],
					[
						Empty, Empty, Empty, Empty, Empty, Empty, Empty, Splitter, Empty, Empty,
						Empty, Empty, Empty, Empty, Empty
					],
					[
						Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
						Empty, Empty, Empty, Empty, Empty
					],
					[
						Empty, Empty, Empty, Empty, Empty, Empty, Splitter, Empty, Splitter, Empty,
						Empty, Empty, Empty, Empty, Empty
					],
					[
						Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
						Empty, Empty, Empty, Empty, Empty
					],
					[
						Empty, Empty, Empty, Empty, Empty, Splitter, Empty, Splitter, Empty,
						Splitter, Empty, Empty, Empty, Empty, Empty
					],
					[
						Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
						Empty, Empty, Empty, Empty, Empty
					],
					[
						Empty, Empty, Empty, Empty, Splitter, Empty, Splitter, Empty, Empty, Empty,
						Splitter, Empty, Empty, Empty, Empty
					],
					[
						Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
						Empty, Empty, Empty, Empty, Empty
					],
					[
						Empty, Empty, Empty, Splitter, Empty, Splitter, Empty, Empty, Empty,
						Splitter, Empty, Splitter, Empty, Empty, Empty
					],
					[
						Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
						Empty, Empty, Empty, Empty, Empty
					],
					[
						Empty, Empty, Splitter, Empty, Empty, Empty, Splitter, Empty, Empty, Empty,
						Empty, Empty, Splitter, Empty, Empty
					],
					[
						Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
						Empty, Empty, Empty, Empty, Empty
					],
					[
						Empty, Splitter, Empty, Splitter, Empty, Splitter, Empty, Splitter, Empty,
						Splitter, Empty, Empty, Empty, Splitter, Empty
					],
					[
						Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
						Empty, Empty, Empty, Empty, Empty
					],
				]
			}
		)
	}

	#[test]
	fn diagram_count_splits_example() {
		let diagram: Diagram<15, 16> = EXAMPLE.parse().unwrap();
		assert_eq!(diagram.count_splits(), 21);
	}
}
