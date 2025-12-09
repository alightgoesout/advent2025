use advent2025::day9::{INPUT, Tile, parse_tiles};
use plotters::prelude::*;
use std::env;
use std::error::Error;

const WIDTH: i32 = 2560;
const HEIGHT: i32 = 1440;

fn read_rectangle_from_args() -> Option<(Tile, Tile)> {
	let args: Vec<_> = env::args().skip(1).take(4).collect();
	if args.len() == 4 {
		Some((
			Tile {
				x: args[0].parse().ok()?,
				y: args[1].parse().ok()?,
			},
			Tile {
				x: args[2].parse().ok()?,
				y: args[3].parse().ok()?,
			},
		))
	} else {
		None
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let tiles = parse_tiles(INPUT).unwrap();
	let (max_x, max_y) = tiles.iter().fold((0, 0), |(max_x, max_y), &Tile { x, y }| {
		(max_x.max(x), max_y.max(y))
	});

	let coordinates = |&Tile { x, y }| {
		(
			x as i32 * (WIDTH - 20) / max_x as i32,
			y as i32 * (HEIGHT - 20) / max_y as i32,
		)
	};

	let mut backend = BitMapBackend::new("day9.png", (WIDTH as u32, HEIGHT as u32));

	draw_tile(&mut backend, coordinates(&tiles[0]))?;

	for (i, tile) in tiles.iter().enumerate().skip(1) {
		draw_tile(&mut backend, coordinates(tile))?;
		backend.draw_line(
			coordinates(&tiles[i - 1]),
			coordinates(tile),
			&ShapeStyle::from(&GREEN),
		)?;
	}

	if let Some((start, end)) = read_rectangle_from_args() {
		backend.draw_rect(
			coordinates(&start),
			coordinates(&end),
			&ShapeStyle::from(&BLUE),
			false,
		)?;
	}

	Ok(())
}

fn draw_tile(backend: &mut BitMapBackend, coordinates: (i32, i32)) -> Result<(), Box<dyn Error>> {
	backend.draw_circle(coordinates, 3, &ShapeStyle::from(&RED), true)?;
	Ok(())
}
