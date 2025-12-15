use itertools::Itertools;
use std::cell::{OnceCell, RefCell};
use std::collections::{HashMap, HashSet};
use std::ops::Index;
use std::str::FromStr;
use winnow::ascii::{digit1, space1};
use winnow::combinator::{delimited, repeat, separated, seq};
use winnow::stream::ParseSlice;
use winnow::token::{one_of, rest};
use winnow::{ModalResult, Parser};

use crate::input::{ParseExt, ReadLines};
use crate::{Error, Result, Solution};

const INPUT: &[u8] = include_bytes!("../input/day10");

#[derive(Default)]
pub struct Day10(OnceCell<Vec<Machine>>);

impl Day10 {
	fn machines(&self) -> Result<&Vec<Machine>> {
		self.0.get_or_try_init(|| parse_machines(INPUT))
	}
}

impl Solution for Day10 {
	fn part_one(&self) -> Result<String> {
		let presses: usize = self
			.machines()?
			.iter()
			.map(Machine::configure_indicator_lights)
			.sum();
		Ok(format!(
			"Number of presses to configure indicator lights: {presses}"
		))
	}

	fn part_two(&self) -> Result<String> {
		let presses: usize = self
			.machines()?
			.iter()
			.map(Machine::configure_joltages)
			.sum();
		Ok(format!(
			"Number of presses to configure joltages: {presses}"
		))
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
	fn apply_on_lights(&self, lights: &[IndicatorLight]) -> Vec<IndicatorLight> {
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

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct Joltage(Vec<usize>);

impl Joltage {
	fn empty(length: usize) -> Self {
		Self(vec![0; length])
	}

	fn len(&self) -> usize {
		self.0.len()
	}

	fn is_all_zeroes(&self) -> bool {
		self.0.iter().all(|i| *i == 0)
	}

	fn is_lower_and_has_same_parity(&self, other: &Joltage) -> bool {
		self.0
			.iter()
			.zip(&other.0)
			.all(|(j1, j2)| j1 <= j2 && j1 % 2 == j2 % 2)
	}

	fn press_button(&mut self, button: &Button) {
		for i in &button.0 {
			self.0[*i] += 1;
		}
	}
}

impl Index<usize> for Joltage {
	type Output = usize;

	fn index(&self, index: usize) -> &Self::Output {
		&self.0[index]
	}
}

#[derive(Debug, Eq, PartialEq)]
struct Machine {
	indicator_lights: Vec<IndicatorLight>,
	buttons: Vec<Button>,
	joltage_requirements: Joltage,
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
					let lights = button.apply_on_lights(&lights);
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

	fn configure_joltages(&self) -> usize {
		JoltagesConfigurator::new(self)
			.compute_minimum_presses(self.joltage_requirements.clone())
			.unwrap()
	}
}

struct JoltagesConfigurator {
	base_joltages: HashMap<Joltage, usize>,
	cache: RefCell<HashMap<Joltage, usize>>,
}

// Solution shamelessly copied from https://www.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/
impl JoltagesConfigurator {
	fn new(machine: &Machine) -> Self {
		let mut base_joltages = HashMap::new();
		for length in 0..=machine.buttons.len() {
			for buttons in machine.buttons.iter().combinations(length) {
				let mut joltage = Joltage::empty(machine.joltage_requirements.len());
				for button in buttons {
					joltage.press_button(button);
				}
				base_joltages.entry(joltage).or_insert(length);
			}
		}
		Self {
			cache: RefCell::default(), /*RefCell::new(base_joltages.clone())*/
			base_joltages,
		}
	}

	fn compute_minimum_presses(&self, joltage: Joltage) -> Option<usize> {
		if let Some(presses) = self.cache.borrow().get(&joltage) {
			return Some(*presses);
		}
		if joltage.is_all_zeroes() {
			return Some(0);
		}
		let mut result = None;
		for (base_joltage, presses) in &self.base_joltages {
			if base_joltage.is_lower_and_has_same_parity(&joltage) {
				let new_joltage = Joltage(
					(0..joltage.len())
						.map(|i| (joltage[i] - base_joltage[i]) / 2)
						.collect(),
				);
				if let Some(new_presses) = self.compute_minimum_presses(new_joltage) {
					let new_presses = presses + 2 * new_presses;
					result = result
						.map(|presses: usize| presses.min(new_presses))
						.or(Some(new_presses));
				}
			}
		}
		result.inspect(|presses| {
			self.cache.borrow_mut().insert(joltage, *presses);
		});
		result
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
		_: space1,
		joltage_requirements: delimited('{', separated(1.., digit1.parse_to::<usize>(), ',') ,'}').map(Joltage),
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
	fn machine_configure_joltages_example_1() {
		let machine = &parse_machines(EXAMPLE).unwrap()[0];
		let presses = machine.configure_joltages();
		assert_eq!(presses, 10);
	}

	#[test]
	fn machine_configure_joltages_example_2() {
		let machine = &parse_machines(EXAMPLE).unwrap()[1];
		let presses = machine.configure_joltages();
		assert_eq!(presses, 12);
	}

	#[test]
	fn machine_configure_joltages_example_3() {
		let machine = &parse_machines(EXAMPLE).unwrap()[2];
		let presses = machine.configure_joltages();
		assert_eq!(presses, 11);
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
					joltage_requirements: Joltage(vec![3, 5, 4, 7]),
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
					joltage_requirements: Joltage(vec![7, 5, 12, 7, 2]),
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
					joltage_requirements: Joltage(vec![10, 11, 11, 5, 10, 5]),
				},
			]
		);
	}
}
