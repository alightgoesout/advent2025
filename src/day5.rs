use std::str::FromStr;

use crate::input::{ParseExt, ReadLines};
use crate::{Error, Result, Solution, error};

const FRESH_INGREDIENTS: &[u8] = include_bytes!("../input/day5_fresh_ingredients");
const AVAILABLE_INGREDIENTS: &[u8] = include_bytes!("../input/day5_available_ingredients");

pub struct Day5;

impl Solution for Day5 {
	fn part_one(&self) -> Result<String> {
		let fresh_ingredients = parse_fresh_ingredients(FRESH_INGREDIENTS)?;
		let available_ingredients = parse_available_ingredients(AVAILABLE_INGREDIENTS)?;
		let available_fresh_ingredients =
			count_available_fresh_ingredients(&fresh_ingredients, &available_ingredients);
		Ok(format!(
			"Number of available fresh ingredients: {available_fresh_ingredients}"
		))
	}

	fn part_two(&self) -> Result<String> {
		todo!()
	}
}

fn count_available_fresh_ingredients(
	fresh_ingredients: &[FreshIngredients],
	available_ingredients: &[u64],
) -> usize {
	available_ingredients
		.iter()
		.filter(|ingredient| {
			fresh_ingredients
				.iter()
				.any(|fresh_ingredients| fresh_ingredients.contains(**ingredient))
		})
		.count()
}

fn parse_fresh_ingredients(input: &[u8]) -> Result<Vec<FreshIngredients>> {
	input.read_lines().parse().collect::<Result<_>>()
}

fn parse_available_ingredients(input: &[u8]) -> Result<Vec<u64>> {
	input.read_lines().parse().collect::<Result<_>>()
}

#[derive(Debug, Eq, PartialEq)]
struct FreshIngredients {
	start: u64,
	end: u64,
}

impl FreshIngredients {
	fn new(start: u64, end: u64) -> Self {
		Self { start, end }
	}

	fn contains(&self, ingredient: u64) -> bool {
		ingredient >= self.start && ingredient <= self.end
	}
}

impl FromStr for FreshIngredients {
	type Err = Error;

	fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
		let (start, end) = s
			.split_once('-')
			.ok_or(error!("Invalid fresh ingredients: {s}"))?;
		Ok(FreshIngredients::new(start.parse()?, end.parse()?))
	}
}

#[cfg(test)]
mod test {
	use super::*;

	const EXAMPLE_FRESH_INGREDIENTS: &[u8] = b"\
3-5
10-14
16-20
12-18
";

	const EXAMPLE_AVAILABLE_INGREDIENTS: &[u8] = b"\
1
5
8
11
17
32
";

	#[test]
	fn parse_fresh_ingredients_example() {
		let result = parse_fresh_ingredients(EXAMPLE_FRESH_INGREDIENTS).unwrap();
		assert_eq!(
			result,
			vec![
				FreshIngredients::new(3, 5),
				FreshIngredients::new(10, 14),
				FreshIngredients::new(16, 20),
				FreshIngredients::new(12, 18),
			],
		);
	}

	#[test]
	fn parse_available_ingredients_example() {
		let result = parse_available_ingredients(EXAMPLE_AVAILABLE_INGREDIENTS).unwrap();

		assert_eq!(result, vec![1, 5, 8, 11, 17, 32]);
	}

	#[test]
	fn count_available_fresh_ingredients_should_return_3_for_example() {
		let fresh_ingredients = parse_fresh_ingredients(EXAMPLE_FRESH_INGREDIENTS).unwrap();
		let available_ingredients =
			parse_available_ingredients(EXAMPLE_AVAILABLE_INGREDIENTS).unwrap();

		let result = count_available_fresh_ingredients(&fresh_ingredients, &available_ingredients);

		assert_eq!(result, 3);
	}
}
