#![feature(once_cell_try)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod error;
mod input;
mod solution;

pub use error::Error;
pub use solution::Solution;
pub type Result<T> = std::result::Result<T, Error>;
pub(crate) use error::error;

pub fn solutions() -> Vec<Box<dyn Solution>> {
	vec![
		Box::new(day1::Day1::default()),
		Box::new(day2::Day2::default()),
		Box::new(day3::Day3::default()),
		Box::new(day4::Day4::default()),
		Box::new(day5::Day5),
	]
}
