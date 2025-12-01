use std::env;

use advent2025::solutions;

fn read_day_from_args() -> Option<usize> {
	env::args()
		.nth(1)
		.map(|arg| arg.parse())
		.transpose()
		.expect("Invalid day")
}

fn main() {
	let solutions = solutions();
	let day = read_day_from_args().unwrap_or(solutions.len());
	if let Some(solution) = solutions.get(day - 1) {
		solution.execute(day as u8).unwrap()
	} else {
		println!("Unknown day {day}")
	}
}
