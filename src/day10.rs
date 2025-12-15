use std::collections::HashSet;
use std::str::FromStr;
use winnow::ascii::{digit1, space1};
use winnow::combinator::{delimited, repeat, separated, seq};
use winnow::stream::ParseSlice;
use winnow::token::{one_of, rest};
use winnow::{ModalResult, Parser};

use crate::input::{ParseExt, ReadLines};
use crate::{Error, Result, Solution};

const INPUT: &[u8] = include_bytes!("../input/day10");

pub struct Day10;

impl Solution for Day10 {
	fn part_one(&self) -> Result<String> {
		let machines = parse_machines(INPUT)?;
		let presses: usize = machines
			.iter()
			.map(Machine::configure_indicator_lights)
			.sum();
		Ok(format!(
			"Number of presses to configure indicator lights: {presses}"
		))
	}

	fn part_two(&self) -> Result<String> {
		todo!()
	}
}

fn parse_machines(input: &[u8]) -> Result<Vec<Machine>> {
	input
		.read_lines()
		.parse::<Machine>()
		.collect::<Result<Vec<_>>>()
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum IndicatorLight {
	On,
	Off,
}

impl IndicatorLight {
	fn toggle(&self) -> Self {
		match self {
			IndicatorLight::On => IndicatorLight::Off,
			IndicatorLight::Off => IndicatorLight::On,
		}
	}
}

impl ParseSlice<IndicatorLight> for char {
	fn parse_slice(&self) -> Option<IndicatorLight> {
		match self {
			'#' => Some(IndicatorLight::On),
			'.' => Some(IndicatorLight::Off),
			_ => None,
		}
	}
}

#[derive(Debug, Eq, PartialEq)]
struct Button(Vec<usize>);

impl Button {
	fn apply(&self, lights: &[IndicatorLight]) -> Vec<IndicatorLight> {
		lights
			.iter()
			.enumerate()
			.map(|(i, light)| {
				if self.0.contains(&i) {
					light.toggle()
				} else {
					*light
				}
			})
			.collect()
	}
}

#[derive(Debug, Eq, PartialEq)]
struct Machine {
	indicator_lights: Vec<IndicatorLight>,
	buttons: Vec<Button>,
}

impl Machine {
	fn configure_indicator_lights(&self) -> usize {
		let mut combinations = vec![(
			vec![IndicatorLight::Off; self.indicator_lights.len()],
			vec![false; self.buttons.len()],
		)];
		let mut explored = HashSet::new();

		loop {
			let mut new_combinations = Vec::new();
			for (lights, presses) in combinations {
				for (i, button) in self.buttons.iter().enumerate() {
					let mut presses = presses.clone();
					presses[i] = true;
					let lights = button.apply(&lights);
					if lights == self.indicator_lights {
						return presses.iter().filter(|b| **b).count();
					} else if !explored.contains(&presses) {
						explored.insert(presses.clone());
						new_combinations.push((lights, presses))
					}
				}
			}
			combinations = new_combinations;
		}
	}
}

impl FromStr for Machine {
	type Err = Error;

	fn from_str(input: &str) -> std::result::Result<Self, Self::Err> {
		let machine = parse_machine.parse(input)?;
		Ok(machine)
	}
}

fn parse_machine(input: &mut &str) -> ModalResult<Machine> {
	seq!(Machine {
		indicator_lights: delimited(
			'[',
			repeat(1.., one_of(['.', '#']).parse_to()),
			']',
		),
		_: space1,
		buttons: separated(
			1..,
			delimited(
				'(',
				separated(1.., digit1.parse_to::<usize>(), ','),
				')',
			)
			.map(Button),
			' ',
		),
		_: rest,
	})
	.parse_next(input)
}

#[cfg(test)]
mod test {
	use super::*;

	const EXAMPLE: &[u8] = b"\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

	macro_rules! button {
    	($($x:expr),+$(,)?) => {Button(vec![$($x),+])};
	}

	#[test]
	fn machine_configure_indicator_lights_example_1() {
		let machine = &parse_machines(EXAMPLE).unwrap()[0];
		let presses = machine.configure_indicator_lights();
		assert_eq!(presses, 2);
	}

	#[test]
	fn machine_configure_indicator_lights_example_2() {
		let machine = &parse_machines(EXAMPLE).unwrap()[1];
		let presses = machine.configure_indicator_lights();
		assert_eq!(presses, 3);
	}

	#[test]
	fn machine_configure_indicator_lights_example_3() {
		let machine = &parse_machines(EXAMPLE).unwrap()[2];
		let presses = machine.configure_indicator_lights();
		assert_eq!(presses, 2);
	}

	#[test]
	fn parse_example() {
		let machines = parse_machines(EXAMPLE).unwrap();

		assert_eq!(
			machines,
			vec![
				Machine {
					indicator_lights: vec![
						IndicatorLight::Off,
						IndicatorLight::On,
						IndicatorLight::On,
						IndicatorLight::Off
					],
					buttons: vec![
						button!(3),
						button!(1, 3),
						button!(2),
						button!(2, 3),
						button!(0, 2),
						button!(0, 1),
					],
				},
				Machine {
					indicator_lights: vec![
						IndicatorLight::Off,
						IndicatorLight::Off,
						IndicatorLight::Off,
						IndicatorLight::On,
						IndicatorLight::Off
					],
					buttons: vec![
						button!(0, 2, 3, 4),
						button!(2, 3),
						button!(0, 4),
						button!(0, 1, 2),
						button!(1, 2, 3, 4),
					],
				},
				Machine {
					indicator_lights: vec![
						IndicatorLight::Off,
						IndicatorLight::On,
						IndicatorLight::On,
						IndicatorLight::On,
						IndicatorLight::Off,
						IndicatorLight::On,
					],
					buttons: vec![
						button!(0, 1, 2, 3, 4),
						button!(0, 3, 4),
						button!(0, 1, 2, 4, 5),
						button!(1, 2),
					],
				},
			]
		);
	}
}
