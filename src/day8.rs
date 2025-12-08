use std::cell::OnceCell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::str::FromStr;

use crate::input::{ParseExt, ReadLines};
use crate::{Error, Result, Solution, error};

const INPUT: &[u8] = include_bytes!("../input/day8");

#[derive(Default)]
pub struct Day8(OnceCell<BinaryHeap<Arc>>);

impl Day8 {
	fn arcs(&self) -> Result<&BinaryHeap<Arc>> {
		self.0.get_or_try_init(|| {
			let positions = parse_positions(INPUT)?;
			Ok(compute_arcs(&positions))
		})
	}
}

impl Solution for Day8 {
	fn part_one(&self) -> Result<String> {
		let (a, b, c) = connect_junction_boxes(self.arcs()?.clone(), 1000)?;
		Ok(format!(
			"Product of the three largest circuits' sizes: {}",
			a * b * c
		))
	}

	fn part_two(&self) -> Result<String> {
		let (first, second) = connect_all_junction_boxes(self.arcs()?.clone(), 1000)?;
		Ok(format!(
			"Product of the X coordinates of the last two connected boxes: {}",
			first.x * second.x
		))
	}
}

fn connect_junction_boxes(mut arcs: BinaryHeap<Arc>, n: usize) -> Result<(usize, usize, usize)> {
	let mut circuits = Vec::new();

	for _ in 0..n {
		if let Some(Arc { first, second, .. }) = arcs.pop() {
			match (
				extract_circuit(&mut circuits, &first),
				extract_circuit(&mut circuits, &second),
			) {
				(Some(mut first_circuit), Some(second_circuit)) => {
					first_circuit.extend(second_circuit);
					first_circuit.insert(second);
					circuits.push(first_circuit);
				}
				(Some(mut circuit), None) => {
					circuit.insert(second);
					circuits.push(circuit);
				}
				(None, Some(mut circuit)) => {
					circuit.insert(first);
					circuits.push(circuit);
				}
				(None, None) => circuits.push(HashSet::from([first, second])),
			}
		}
	}

	circuits.sort_by_key(|circuit| std::cmp::Reverse(circuit.len()));
	let mut iter = circuits.iter().map(HashSet::len).take(3);
	Ok((
		iter.next().ok_or_else(|| error!("No circuits"))?,
		iter.next().ok_or_else(|| error!("Only one circuit"))?,
		iter.next().ok_or_else(|| error!("Only two"))?,
	))
}

fn extract_circuit(
	circuits: &mut Vec<HashSet<Position>>,
	position: &Position,
) -> Option<HashSet<Position>> {
	circuits
		.iter()
		.position(|circuit| circuit.contains(position))
		.map(|index| circuits.remove(index))
}

fn connect_all_junction_boxes(
	mut arcs: BinaryHeap<Arc>,
	nb_boxes: usize,
) -> Result<(Position, Position)> {
	let mut circuit = HashSet::new();

	while let Some(Arc { first, second, .. }) = arcs.pop() {
		circuit.insert(first);
		circuit.insert(second);
		if circuit.len() == nb_boxes {
			return Ok((first, second));
		}
	}

	Err(error!("Could not connect all boxes"))
}

fn compute_arcs(positions: &[Position]) -> BinaryHeap<Arc> {
	let mut arcs = BinaryHeap::new();
	for (i, first) in positions.iter().enumerate() {
		for second in &positions[(i + 1)..] {
			arcs.push(Arc::new(*first, *second))
		}
	}
	arcs
}

fn parse_positions(input: &[u8]) -> Result<Vec<Position>> {
	input.read_lines().parse().collect()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Position {
	x: u64,
	y: u64,
	z: u64,
}

impl Position {
	fn new(x: u64, y: u64, z: u64) -> Self {
		Self { x, y, z }
	}

	fn distance(&self, other: &Position) -> f64 {
		((self.x.abs_diff(other.x).pow(2)
			+ self.y.abs_diff(other.y).pow(2)
			+ self.z.abs_diff(other.z).pow(2)) as f64)
			.sqrt()
	}
}

impl FromStr for Position {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self> {
		let parts = s.split(',').parse().collect::<Result<Vec<_>>>()?;
		if parts.len() != 3 {
			return Err(error!("Invalid position {s}"));
		}
		Ok(Position::new(parts[0], parts[1], parts[2]))
	}
}

#[derive(Debug, Copy, Clone)]
struct Arc {
	distance: f64,
	first: Position,
	second: Position,
}

impl Arc {
	fn new(first: Position, second: Position) -> Self {
		Self {
			distance: first.distance(&second),
			first,
			second,
		}
	}
}

impl Eq for Arc {}

impl PartialEq for Arc {
	fn eq(&self, other: &Self) -> bool {
		self.first == other.first && self.second == other.second
	}
}

impl Ord for Arc {
	fn cmp(&self, other: &Self) -> Ordering {
		self.distance
			.partial_cmp(&other.distance)
			.unwrap_or(Ordering::Equal)
			.reverse()
	}
}

impl PartialOrd for Arc {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

#[cfg(test)]
mod test {
	use super::*;

	const EXAMPLE: &[u8] = b"\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

	#[test]
	fn parse_position() {
		let position: Position = "162,817,812".parse().unwrap();
		assert_eq!(position, Position::new(162, 817, 812));
	}

	#[test]
	fn position_distance_should_return_0_for_same_position() {
		let position = Position::new(1, 2, 3);
		assert_eq!(position.distance(&position), 0.0);
	}

	#[test]
	fn position_distance_should_return_sqrt3_for_0_0_0_and_1_1_1() {
		let position1 = Position::new(0, 0, 0);
		let position2 = Position::new(1, 1, 1);
		assert_eq!(position1.distance(&position2), 3f64.sqrt());
		assert_eq!(position2.distance(&position1), 3f64.sqrt());
	}

	#[test]
	fn compute_arcs_2_positions() {
		let positions = vec![Position::new(0, 0, 0), Position::new(1, 1, 1)];
		assert_eq!(
			compute_arcs(&positions).into_vec(),
			vec![Arc::new(Position::new(0, 0, 0), Position::new(1, 1, 1))],
		);
	}

	#[test]
	fn compute_arcs_example() {
		let positions = parse_positions(EXAMPLE).unwrap();

		let mut result = compute_arcs(&positions);

		assert_eq!(
			result.pop(),
			Some(Arc::new(
				Position::new(162, 817, 812),
				Position::new(425, 690, 689),
			))
		);
		assert_eq!(
			result.pop(),
			Some(Arc::new(
				Position::new(162, 817, 812),
				Position::new(431, 825, 988),
			))
		);
	}

	#[test]
	fn connect_junction_boxes_example() {
		let positions = parse_positions(EXAMPLE).unwrap();
		let arcs = compute_arcs(&positions);

		let result = connect_junction_boxes(arcs, 10).unwrap();

		assert_eq!(result, (5, 4, 2))
	}

	#[test]
	fn connect_all_junction_boxes_example() {
		let positions = parse_positions(EXAMPLE).unwrap();
		let arcs = compute_arcs(&positions);

		let result = connect_all_junction_boxes(arcs, positions.len()).unwrap();

		assert_eq!(
			result,
			(Position::new(216, 146, 977), Position::new(117, 168, 530)),
		)
	}
}
