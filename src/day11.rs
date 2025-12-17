use std::cell::OnceCell;
use std::collections::HashMap;
use winnow::ascii::{alpha1, line_ending};
use winnow::combinator::{opt, separated};
use winnow::error::ContextError;
use winnow::{Parser, seq};

use crate::{Result, Solution};

const INPUT: &str = include_str!("../input/day11");

#[derive(Default)]
pub struct Day11(OnceCell<HashMap<&'static str, Device<'static>>>);

impl Day11 {
	fn devices(&self) -> Result<&HashMap<&'static str, Device<'static>>> {
		self.0.get_or_try_init(|| parse_devices(INPUT))
	}
}

impl Solution for Day11 {
	fn part_one(&self) -> Result<String> {
		let nb_paths = count_paths_from_you_to_out(self.devices()?);
		Ok(format!("Number of paths from you to out: {nb_paths}"))
	}

	fn part_two(&self) -> Result<String> {
		let nb_paths = count_paths_from_svr_to_out(self.devices()?);
		Ok(format!("Number of paths from svr to out: {nb_paths}"))
	}
}

fn count_paths_from_you_to_out<'a>(devices: &HashMap<&'a str, Device<'a>>) -> usize {
	count_paths(devices, "you", "out")
}

fn count_paths_from_svr_to_out<'a>(devices: &HashMap<&'a str, Device<'a>>) -> usize {
	let (first, second, middle_paths) = {
		let middle_paths = count_paths(devices, "dac", "fft");
		if middle_paths == 0 {
			("fft", "dac", count_paths(devices, "fft", "dac"))
		} else {
			("dac", "fft", middle_paths)
		}
	};
	let start_paths = count_paths(devices, "svr", first);
	let end_paths = count_paths(devices, second, "out");
	start_paths * middle_paths * end_paths
}

fn count_paths<'a>(
	devices: &HashMap<&'a str, Device<'a>>,
	start_device: &'a str,
	end_device: &str,
) -> usize {
	let mut paths_to_end = HashMap::from([(end_device, 1)]);
	let mut stack = devices
		.get(start_device)
		.map(|start_device| vec![(start_device.name, start_device.output.as_slice(), 0)])
		.unwrap_or_default();

	while let Some((device, to_explore, nb_paths)) = stack.pop() {
		if let Some((first_to_explore, rest_to_explore)) = to_explore.split_first() {
			if let Some(p) = paths_to_end.get(first_to_explore) {
				stack.push((device, rest_to_explore, nb_paths + *p))
			} else if let Some(device_to_explore) = devices.get(first_to_explore) {
				stack.push((device, to_explore, nb_paths));
				stack.push((
					device_to_explore.name,
					device_to_explore.output.as_slice(),
					0,
				));
			} else {
				paths_to_end.insert(first_to_explore, 0);
			}
		} else {
			paths_to_end.insert(device, nb_paths);
		}
	}

	paths_to_end[&start_device]
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

	const EXAMPLE_1: &str = "\
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

	const EXAMPLE_2: &str = "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
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
	fn compute_paths_from_you_to_out_example_1() {
		let devices = parse_devices(EXAMPLE_1).unwrap();
		let result = count_paths_from_you_to_out(&devices);
		assert_eq!(result, 5);
	}

	#[test]
	fn compute_paths_to_out_example_2() {
		let devices = parse_devices(EXAMPLE_2).unwrap();
		let result = count_paths_from_svr_to_out(&devices);
		assert_eq!(result, 2);
	}

	#[test]
	fn parse_example_1_test() {
		let devices = parse_devices(EXAMPLE_1).unwrap();
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
