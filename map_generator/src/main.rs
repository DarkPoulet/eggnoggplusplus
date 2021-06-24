#![allow(unused_parens)]
#![allow(dead_code)]

// use nformation::*;
pub mod information;
use information::*;

pub mod tile;
use tile::*;

pub mod panel;
use panel::*;

pub mod inout;
use inout::*;

pub mod map;
use map::*;

fn main()
{
	let nb_panels = 6;

	let mut map = Map::new(nb_panels);
	randomize_map(&mut map);

	map.set(nb_panels - 1, create_last_panel(5));
	map.set(0, generate_panel_heightmap());
	// map.set(0, test_panel());

	let folder_for_this_map = "latest/".to_string();
	let path = format!("{}{}", information::SAVING_PATH, folder_for_this_map);
	// let filename = format!("{}{}", path, "generated_0.map");
	let map_name = "generated_11.map".to_string();
	if let Err(e) = map.save(path, map_name)
	{
		println!("Error saving map: {}!", e);
	}


	
	let first_panel = map.get(0);
	let clingo_model = ask_clingo(&first_panel);
	let panel_with_path = from_clingo(clingo_model);

	println!("Map:\n{}\n\nMap with path:\n{}\n", first_panel, panel_with_path);

	// ask_clingo(&map.get(0));


	// println!("{}", map);
}
