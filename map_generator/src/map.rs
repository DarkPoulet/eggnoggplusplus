use std::fmt::Display;
use std::io::{Error};

use crate::information;
use crate::information::*;
use crate::inout;
use crate::inout::*;
use crate::panel;
use crate::panel::*;
use crate::tile;
use crate::tile::*;

pub struct Map
{
	nb_panels: usize,
	panels: Vec<Panel>,
	name: String,
}

impl Map
{
	// initialize new map with uninitialized panels
	pub fn new(nb_panels: usize) -> Self
	{
		let panels = vec![Panel::new(); nb_panels];
		let map = Map {
			nb_panels: nb_panels,
			panels: panels,
			name: "Generated#BipBop".to_string(),
		};
		map
	}

	pub fn nb_panels(&self) -> usize
	{
		self.nb_panels
	}

	// get panel of index 'which_panel'
	pub fn get(&self, which_panel: usize) -> Panel
	{
		self.panels[which_panel].clone()
	}

	pub fn set(&mut self, which_panel: usize, panel: Panel) -> ()
	{
		self.panels[which_panel] = panel;
	}

	// Saves the map to file 'filename' at 'path'
	pub fn save(&self, path: String, filename: String) -> Result<(), Error>
	{
		inout::save_to_file(format!("{}", self), path, filename)
	}
}

impl Display for Map
{
	// display the map in the 'map' format, ready to be read by other program.
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
	{
		write!(f, "{}\n", self.name)?;
		write!(f, "{}\n", information::SEPARATOR_PANELS)?;
		// let mut first = true;

		for panel in &self.panels
		{
			write!(f, "{}\n", panel)?;
			write!(f, "{}\n", information::SEPARATOR_PANELS)?;
		}
		Ok(())
	}
}

pub fn replace_undefined_by_air_in_map(map: &mut Map) -> ()
{
	for i in 0..map.nb_panels()
	{
		let mut panel_i = map.get(i);
		replace_undefined_by_air_in_panel(&mut panel_i);
		map.set(i, panel_i)
	}
}
pub fn randomize_map(map: &mut Map) -> ()
{
	for i in 0..map.nb_panels()
	{
		let random_panel = generate_random_panel();
		map.set(i, random_panel);
	}
}
