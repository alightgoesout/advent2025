use std::cell::OnceCell;
use std::cmp::Ordering;
use std::str::FromStr;

use crate::input::{ParseExt, ReadLines};
use crate::{Error, Result, Solution, error};

const FRESH_INGREDIENTS: &[u8] = include_bytes!("../input/day5_fresh_ingredients");
const AVAILABLE_INGREDIENTS: &[u8] = include_bytes!("../input/day5_available_ingredients");

#[derive(Default)]
pub struct Day5(OnceCell<Vec<FreshIngredients>>);

impl Day5 {
	fn fresh_ingredients(&self) -> Result<&Vec<FreshIngredients>> {
		self.0
			.get_or_try_init(|| parse_fresh_ingredients(FRESH_INGREDIENTS))
	}
}

impl Solution for Day5 {
	fn part_one(&self) -> Result<String> {
		let available_ingredients = parse_available_ingredients(AVAILABLE_INGREDIENTS)?;
		let available_fresh_ingredients =
			count_available_fresh_ingredients(self.fresh_ingredients()?, &available_ingredients);
		Ok(format!(
			"Number of available fresh ingredients: {available_fresh_ingredients}"
		))
	}

	fn part_two(&self) -> Result<String> {
		let nb_fresh_ids = count_fresh_ids(self.fresh_ingredients()?);
		Ok(format!("Number of fresh ingredient IDs: {nb_fresh_ids}"))
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

fn count_fresh_ids(fresh_ingredients: &[FreshIngredients]) -> u64 {
	let mut fresh_ingredients = fresh_ingredients.to_vec();
	fresh_ingredients.sort();

	let mut fresh_ingredients_without_overlap = Vec::<FreshIngredients>::new();

	for current in fresh_ingredients {
		match fresh_ingredients_without_overlap.pop() {
			Some(previous) => match previous.merge(current) {
				Some(merged) => fresh_ingredients_without_overlap.push(merged),
				None => {
					fresh_ingredients_without_overlap.push(previous);
					fresh_ingredients_without_overlap.push(current)
				}
			},
			None => fresh_ingredients_without_overlap.push(current),
		}
	}

	fresh_ingredients_without_overlap
		.iter()
		.map(FreshIngredients::len)
		.sum()
}

fn parse_fresh_ingredients(input: &[u8]) -> Result<Vec<FreshIngredients>> {
	input.read_lines().parse().collect::<Result<_>>()
}

fn parse_available_ingredients(input: &[u8]) -> Result<Vec<u64>> {
	input.read_lines().parse().collect::<Result<_>>()
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
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

	fn len(&self) -> u64 {
		self.end - self.start + 1
	}

	fn merge(self, rhs: FreshIngredients) -> Option<FreshIngredients> {
		if self.contains(rhs.start) {
			Some(FreshIngredients::new(self.start, self.end.max(rhs.end)))
		} else if rhs.contains(self.start) {
			Some(FreshIngredients::new(rhs.start, self.end.max(rhs.end)))
		} else {
			None
		}
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

impl Ord for FreshIngredients {
	fn cmp(&self, other: &Self) -> Ordering {
		self.start
			.cmp(&other.start)
			.then_with(|| self.end.cmp(&other.end))
	}
}

impl PartialOrd for FreshIngredients {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
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

	#[test]
	fn count_fresh_ids_should_return_14_for_example() {
		let fresh_ingredients = parse_fresh_ingredients(EXAMPLE_FRESH_INGREDIENTS).unwrap();
		assert_eq!(count_fresh_ids(&fresh_ingredients), 14);
	}
}
