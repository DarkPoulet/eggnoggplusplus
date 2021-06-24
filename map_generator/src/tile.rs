#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile
{
	Undefined,
	Ground,
	Air,
	DownSpikes,
	UpSpikes,
	Mine,
	Sword,
	SpikyBall,
	EggnoggWave,
	Eggnogg,
	Wave,
	Wavee,
}

impl Tile
{
	pub fn to_str(self) -> &'static str
	{
		match self
		{
			Tile::Ground => "@",
			Tile::Air => " ",
			Tile::Undefined => "?",
			Tile::DownSpikes => "v",
			Tile::UpSpikes => "X",
			Tile::Mine => "m",
			Tile::Sword => "*",
			Tile::SpikyBall => "K",
			Tile::EggnoggWave => "^",
			Tile::Eggnogg => "E",
			Tile::Wave => "w",
			Tile::Wavee => "W",
		}
	}
}
