use std::fs::File;
use std::io::{Error, Write};
use std::fs;

pub fn save_to_file<T, TT, TTT>(text: T, path: TT, filename: TTT) -> Result<(), Error>
where 
T : Into<String>, T : Clone, 
TT : Into<String>, TT : Clone, 
TTT : Into<String>, TTT : Clone
{
	fs::create_dir_all(path.clone().into())?;
	let mut output = File::create(&format!("{}{}", path.into(), filename.into()))?;
	write!(output, "{}", text.into())?;
	Ok(())
}
