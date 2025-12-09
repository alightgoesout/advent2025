use itertools::Itertools;
use std::cell::OnceCell;
use std::str::FromStr;

use crate::error::error;
use crate::input::{ParseExt, ReadLines};
use crate::{Error, Result, Solution};

pub const INPUT: &[u8] = include_bytes!("../input/day9");

#[derive(Default)]
pub struct Day9(OnceCell<Vec<Tile>>);

impl Day9 {
	fn tiles(&self) -> Result<&Vec<Tile>> {
		self.0.get_or_try_init(|| parse_tiles(INPUT))
	}
}

impl Solution for Day9 {
	fn part_one(&self) -> Result<String> {
		let largest_rectangle_area = find_largest_rectangle_area(self.tiles()?);
		Ok(format!("Largest rectangle area: {largest_rectangle_area}"))
	}

	fn part_two(&self) -> Result<String> {
		let largest_red_and_green_rectangle =
			find_largest_red_and_green_rectangle(self.tiles()?)
				.ok_or_else(|| error!("Could not find any rectangle"))?;
		Ok(format!(
			"Largest red and green rectangle : {:?} {:?} (area: {})",
			largest_red_and_green_rectangle.top_left,
			largest_red_and_green_rectangle.bottom_right,
			largest_red_and_green_rectangle.area(),
		))
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

fn find_largest_red_and_green_rectangle(tiles: &[Tile]) -> Option<Rectangle> {
	let segments = compute_segments(tiles);
	tiles
		.iter()
		.cartesian_product(tiles)
		.map(Rectangle::from)
		.sorted_by_key(Rectangle::area)
		.rev()
		.find(|rectangle| {
			segments
				.iter()
				.all(|segment| !segment.intersects(rectangle))
		})
}

fn compute_segments(tiles: &[Tile]) -> Vec<Segment> {
	tiles
		.iter()
		.chain(vec![&tiles[0]])
		.tuple_windows()
		.map(|(first, second)| Segment::new(first, second))
		.collect::<Vec<_>>()
}

pub fn parse_tiles(input: &[u8]) -> Result<Vec<Tile>> {
	input.read_lines().parse().collect()
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Tile {
	pub x: u64,
	pub y: u64,
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

struct Segment {
	start: Tile,
	end: Tile,
}

impl Segment {
	fn new(first: &Tile, second: &Tile) -> Self {
		if first.x == second.x {
			Self {
				start: Tile::new(first.x, first.y.min(second.y)),
				end: Tile::new(first.x, first.y.max(second.y)),
			}
		} else {
			Self {
				start: Tile::new(first.x.min(second.x), first.y),
				end: Tile::new(first.x.max(second.x), first.y),
			}
		}
	}

	fn intersects(&self, rectangle: &Rectangle) -> bool {
		self.start.x < rectangle.bottom_right.x
			&& self.start.y < rectangle.bottom_right.y
			&& self.end.x > rectangle.top_left.x
			&& self.end.y > rectangle.top_left.y
	}
}

struct Rectangle {
	top_left: Tile,
	bottom_right: Tile,
}

impl Rectangle {
	fn new(first: &Tile, second: &Tile) -> Self {
		Self {
			top_left: Tile::new(first.x.min(second.x), first.y.min(second.y)),
			bottom_right: Tile::new(first.x.max(second.x), first.y.max(second.y)),
		}
	}

	fn area(&self) -> u64 {
		self.top_left.rectangle_area(&self.bottom_right)
	}
}

impl From<(&Tile, &Tile)> for Rectangle {
	fn from((first, second): (&Tile, &Tile)) -> Self {
		Self::new(first, second)
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

	#[test]
	fn find_largest_red_and_green_rectangle_should_return_24_for_example() {
		let tiles = parse_tiles(EXAMPLE).unwrap();
		let rectangle = find_largest_red_and_green_rectangle(&tiles).unwrap();
		assert_eq!(rectangle.area(), 24);
	}
}
