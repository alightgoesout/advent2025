use std::str::FromStr;

use crate::error::error;
use crate::input::{ParseExt, ReadLines};
use crate::{Error, Result, Solution};

const INPUT: &[u8] = include_bytes!("../input/day9");

pub struct Day9;

impl Solution for Day9 {
	fn part_one(&self) -> Result<String> {
		let tiles = parse_tiles(INPUT)?;
		let largest_rectangle_area = find_largest_rectangle_area(&tiles);
		Ok(format!("Largest rectangle area: {largest_rectangle_area}"))
	}

	fn part_two(&self) -> Result<String> {
		todo!()
	}
}

fn find_largest_rectangle_area(tiles: &[Tile]) -> u64 {
	let mut largest_rectangle_area = 0;

	for (i, first) in tiles.iter().enumerate() {
		for second in &tiles[i + 1..] {
			largest_rectangle_area = largest_rectangle_area.max(first.rectangle_area(second));
		}
	}

	largest_rectangle_area
}

fn parse_tiles(input: &[u8]) -> Result<Vec<Tile>> {
	input.read_lines().parse().collect()
}

#[derive(Debug, Eq, PartialEq)]
struct Tile {
	x: u64,
	y: u64,
}

impl Tile {
	fn new(x: u64, y: u64) -> Self {
		Self { x, y }
	}

	fn rectangle_area(&self, other: &Tile) -> u64 {
		(self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
	}
}

impl FromStr for Tile {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self> {
		let (x, y) = s
			.split_once(',')
			.ok_or_else(|| error!("Invalid tile {s}"))?;
		Ok(Self::new(x.parse()?, y.parse()?))
	}
}

#[cfg(test)]
mod test {
	use super::*;

	const EXAMPLE: &[u8] = b"\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

	#[test]
	fn parse_tiles_example() {
		let tiles = parse_tiles(EXAMPLE).unwrap();
		assert_eq!(
			tiles,
			vec![
				Tile::new(7, 1),
				Tile::new(11, 1),
				Tile::new(11, 7),
				Tile::new(9, 7),
				Tile::new(9, 5),
				Tile::new(2, 5),
				Tile::new(2, 3),
				Tile::new(7, 3),
			],
		);
	}

	#[test]
	fn tile_area_should_return_50_for_2_5_and_11_1() {
		assert_eq!(
			Tile { x: 2, y: 5 }.rectangle_area(&Tile { x: 11, y: 1 }),
			50
		);
	}

	#[test]
	fn find_largest_rectangle_area_should_return_50_for_example() {
		let tiles = parse_tiles(EXAMPLE).unwrap();
		assert_eq!(find_largest_rectangle_area(&tiles), 50);
	}
}
