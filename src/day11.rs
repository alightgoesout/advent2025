use std::collections::{HashMap, HashSet};
use winnow::ascii::{alpha1, line_ending};
use winnow::combinator::{opt, separated};
use winnow::error::ContextError;
use winnow::{Parser, seq};

use crate::{Result, Solution};

const INPUT: &str = include_str!("../input/day11");

pub struct Day11;

impl Solution for Day11 {
	fn part_one(&self) -> Result<String> {
		let devices = parse_devices(INPUT)?;
		let paths = compute_paths_to_out(&devices);
		Ok(format!("Number of paths to out: {}", paths.len()))
	}

	fn part_two(&self) -> Result<String> {
		todo!()
	}
}

fn compute_paths_to_out<'a>(devices: &HashMap<&'a str, Device<'a>>) -> HashSet<Vec<&'a str>> {
	let mut paths_to_out = HashSet::new();
	let mut paths_to_explore = vec![vec!["you"]];

	while let Some(path) = paths_to_explore.pop() {
		let device = &devices[path.last().unwrap()];
		for output in &device.output {
			let mut new_path = path.clone();
			new_path.push(output);
			if *output == "out" {
				paths_to_out.insert(new_path);
			} else {
				paths_to_explore.push(new_path);
			}
		}
	}

	paths_to_out
}

fn parse_devices(input: &str) -> Result<HashMap<&str, Device<'_>>> {
	let (devices,): (Vec<_>,) = Parser::<_, _, ContextError>::parse(
		&mut seq!(
			separated(
				0..,
				seq!(
					Device {
						name: alpha1,
						_: ": ",
						output: separated(1.., alpha1, ' '),
					}
				),
				line_ending,
			),
			_: opt(line_ending)
		),
		input,
	)?;
	Ok(devices
		.into_iter()
		.map(|device: Device| (device.name, device))
		.collect())
}

#[derive(Debug, Eq, PartialEq)]
struct Device<'a> {
	name: &'a str,
	output: Vec<&'a str>,
}

#[cfg(test)]
mod test {
	use super::*;

	const EXAMPLE: &str = "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

	macro_rules! device {
		($name: expr => $($o:expr),+$(,)?) => {
			crate::day11::Device {
				name: $name,
				output: vec![$($o),+],
			}
		};
	}

	#[test]
	fn compute_paths_to_out_example() {
		let devices = parse_devices(EXAMPLE).unwrap();

		let result = compute_paths_to_out(&devices);

		assert_eq!(
			result,
			HashSet::from([
				vec!["you", "bbb", "ddd", "ggg", "out"],
				vec!["you", "bbb", "eee", "out"],
				vec!["you", "ccc", "ddd", "ggg", "out"],
				vec!["you", "ccc", "eee", "out"],
				vec!["you", "ccc", "fff", "out"],
			])
		);
	}

	#[test]
	fn parse_example_test() {
		let devices = parse_devices(EXAMPLE).unwrap();
		assert_eq!(
			devices,
			HashMap::from([
				("aaa", device!("aaa" => "you", "hhh")),
				("you", device!("you" => "bbb", "ccc")),
				("bbb", device!("bbb" => "ddd", "eee")),
				("ccc", device!("ccc" => "ddd", "eee", "fff")),
				("ddd", device!("ddd" => "ggg")),
				("eee", device!("eee" => "out")),
				("fff", device!("fff" => "out")),
				("ggg", device!("ggg" => "out")),
				("hhh", device!("hhh" => "ccc", "fff", "iii")),
				("iii", device!("iii" => "out")),
			])
		);
	}
}
