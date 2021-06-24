use std::fmt::Display;
use rand::Rng;
use std::process::Command;


use crate::information::*;
use crate::inout;
use crate::tile::*;

use regex::Regex;

#[derive(Debug, Clone)]
pub struct Panel
{
	tiles: [[Tile; WIDTH]; HEIGHT],
}

impl Panel
{
	// init new panel with UNDEFINED tiles.
	pub fn new() -> Self
	{
		let one_row = [Tile::Undefined; WIDTH];
		let tiles_panel = [one_row; HEIGHT];

		let p = Panel { tiles: tiles_panel };
		p
	}

	// Set a tile. The tile (0,0) is the bottom-left.
	pub fn set(&mut self, x: usize, y: usize, tile: Tile) -> ()
	{
		self.tiles[(HEIGHT - 1) - y][x] = tile;
	}

	pub fn set_row(&mut self, y: usize, tiles: [Tile; WIDTH]) -> ()
	{
		for (x, tile) in tiles.iter().enumerate()
		{
			self.set(x, y, *tile);
		}
	}

	pub fn set_column(&mut self, x: usize, tiles: [Tile; HEIGHT]) -> ()
	{
		for (y, tile) in tiles.iter().enumerate()
		{
			self.set(x, y, *tile);
		}
	}

	// set a whole column from one tile
	pub fn set_column_one_tile(&mut self, x: usize, tile: Tile) -> ()
	{
		let tiles = [tile; HEIGHT];
		self.set_column(x, tiles);
	}

	pub fn set_row_one_tile(&mut self, y: usize, tile: Tile) -> ()
	{
		let tiles = [tile; WIDTH];
		self.set_row(y, tiles);
	}

	pub fn get(&self, x: usize, y: usize) -> Tile
	{
		self.tiles[(HEIGHT - 1) - y][x]
	}
}

impl Display for Panel
{
	// display the panel as in the 'map' format
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
	{
		let mut first = true;

		for row in &self.tiles
		{
			if !first
			{
				write!(f, "\n")?;
			}
			else
			{
				first = false;
			}
			for tile in row.iter()
			{
				write!(f, "{}", tile.to_str())?;
			}
		}
		Ok(())
	}
}

// Creates basic victory panel with water below level 'height'.
pub fn create_last_panel(height: usize) -> Panel
{
	assert!(height > 0 && height < HEIGHT);
	let mut p = Panel::new();

	// fills the eggnogg waves and the ground
	for y in 0..height
	{
		p.set_row_one_tile(y, Tile::EggnoggWave);
		for x in 0..5
		{
			p.set(WIDTH - 1 - x, y, Tile::Ground);
		}
	}
	// fills the air
	for y in height..HEIGHT
	{
		p.set_row_one_tile(y, Tile::Air);
	}
	p
}

pub fn replace_undefined_by_air_in_panel(p: &mut Panel) -> ()
{
	for y in 0..HEIGHT
	{
		for x in 0..WIDTH
		{
			if p.get(x, y) == Tile::Undefined
			{
				p.set(x, y, Tile::Air);
			}
		}
	}
}

pub fn generate_random_panel() -> Panel
{
	let mut p = Panel::new();
	for y in 0..HEIGHT
	{
		for x in 0..WIDTH
		{
			let num_tile = rand::thread_rng().gen_range(0..99 + 1);
			let tile = match num_tile
			{
				0..=24 => Tile::Ground,
				25..=99 => Tile::Air,
				_ => Tile::Undefined,
			};
			p.set(x, y, tile);
		}
	}
	p
}

#[derive(PartialEq, Eq, Clone)]
enum HeightActions
{
	Up,
	Down,
	Stay,
}

pub fn generate_panel_heightmap() -> Panel
{
	let mut p = Panel::new();

	let effective_height = HEIGHT - 2;

	for y in effective_height..HEIGHT
	{
		p.set_row_one_tile(y, Tile::Ground);
	}

	let starting_height = 3;
	let finishing_height = 6;
	let mut heights = [0; WIDTH];
	heights[0] = starting_height;
	heights[WIDTH - 1] = finishing_height;

	let mut current_height = starting_height;
	for x in 1..WIDTH - 1
	{
		let random = rand::thread_rng().gen_range(0..99 + 1);
		let height_action = match random
		{
			0..=19 => HeightActions::Up,
			20..=39 => HeightActions::Down,
			_ => HeightActions::Stay,
		};
		if height_action != HeightActions::Stay
		{
			let random_diff = rand::thread_rng().gen_range(1..4 + 1);
			if height_action == HeightActions::Up && current_height + random_diff < effective_height
			{
				current_height += random_diff
			}
			else if height_action == HeightActions::Down && current_height > random_diff
			{
				current_height -= random_diff
			}
		}
		heights[x] = current_height;
	}

	println!("{:?}", heights);

	for y in 0..effective_height
	{
		for x in 0..WIDTH
		{
			let random = rand::thread_rng().gen_range(0..99 + 1);
			if heights[x] < y || random < 0
			{
				p.set(x, y, Tile::Air);
			}
			else
			{
				p.set(x, y, Tile::Ground);
			}
		}
	}
	// place_random_elements_on_panel(&mut p);
	p
}

pub fn place_random_elements_on_panel(panel: &mut Panel) -> ()
{
	let effective_height = HEIGHT - 2;
	let how_many = 5;
	for _ in 0..how_many
	{
		let x = rand::thread_rng().gen_range(0..WIDTH);
		let y = rand::thread_rng().gen_range(0..effective_height);
		let random = rand::thread_rng().gen_range(0..99 + 1);
		let tile = match random
		{
			0..=19 => Tile::DownSpikes,
			20..=39 => Tile::UpSpikes,
			40..=59 => Tile::Mine,
			60..=79 => Tile::SpikyBall,
			_ => Tile::Mine,
		};
		panel.set(x, y, tile);
	}
}

pub fn to_clingo(panel: &Panel) -> String
{
	let mut modelisation = 
		"#const width=33.\n#const height=12.\n#const begin_height=6.\n#const end_height=8.\n".to_string();
	for y in 0..HEIGHT
	{
		for x in 0..WIDTH
		{
			if panel.get(x, y) == Tile::Ground
			{
				modelisation = format!("{}\nblock({x}, {y}).", modelisation, x = x, y = y);
			}
		}
	}
	modelisation
}

pub fn ask_clingo(panel : &Panel) -> String
{
	const PATH_CLINGO : &'static str = "clingo/"; 
	let filename_input = "input.lp";
	let input_clingo = to_clingo(&panel);
	let _ = inout::save_to_file(input_clingo, PATH_CLINGO, filename_input);

	let file_input = format!("{}{}", PATH_CLINGO, filename_input);
	let file_encoding = format!("{}{}", PATH_CLINGO, "path.lp");

	let output = Command::new("clingo")
        .arg(file_encoding)
        .arg(file_input)
        .output()
        .expect("clingo failure");

		
	let model_clingo = String::from_utf8_lossy(&output.stdout).into_owned();

	model_clingo
	// println!("model clingo:\n{}", model_clingo);
}


pub fn from_clingo(output_clingo : String) -> Panel
{
	let mut p = Panel::new();
	replace_undefined_by_air_in_panel(&mut p);

	let regex_block = Regex::new(r"block\((\d+),(\d+)\)").unwrap();
	let regex_path = Regex::new(r"path\((\d+),(\d+),\d\)").unwrap();
	for cap in regex_block.captures_iter(&output_clingo) 
	{
		// println!("x: {} -  y: {}", &cap[1], &cap[2]);
		let x = cap[1].parse::<usize>().unwrap();
		let y = cap[2].parse::<usize>().unwrap();
		p.set(x, y, Tile::Ground);
	}
	for cap in regex_path.captures_iter(&output_clingo) 
	{
		let x = cap[1].parse::<usize>().unwrap();
		let y = cap[2].parse::<usize>().unwrap();
		p.set(x, y, Tile::Undefined);
	}
	p
}



pub fn test_panel() -> Panel
{
	let mut p = Panel::new();
	p.set_row_one_tile(3, Tile::Ground);
	p.set(10, 4, Tile::Ground);
	p.set(10, 5, Tile::Ground);
	p.set(10, 6, Tile::Ground);
	p.set(10, 7, Tile::Ground);
	p.set(10, 8, Tile::Ground);
	replace_undefined_by_air_in_panel(&mut p);
	p
}