use std::{collections::HashMap, error::Error, io::Write, result::Result as StdResult};

use crate::{cli::Args, config::KanaTable, parser::machine::Machine};

pub const fn load_kanas() -> (&'static str, &'static str) {
	(
		include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/hiragana.toml")),
		include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/katakana.toml")),
	)
}

pub type Result = StdResult<(), Box<dyn Error>>;

pub fn run<O>(mut out: O, args: Args) -> Result
where
	O: Write,
{
	let words = args.words.join(" ");
	let (hiragana, katakana) = load_kanas();
	let (hiragana, katakana) = (toml::de::from_str(hiragana)?, toml::de::from_str(katakana)?);

	let tables = {
		let mut m = HashMap::<bool, &KanaTable>::new();
		m.insert(false, &hiragana);
		m.insert(true, &katakana);
		m
	};

	Machine::start(
		&tables,
		(args.katakana, args.toggle_char),
		&words,
		|result| writeln!(out, "{}", result),
	)?;

	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_output_hiragana() -> Result {
		let mut out = Vec::new();
		let args = Args {
			words: Vec::from(["arigatougozaimasu!".into(), "itadakimasu!".into()]),
			..Default::default()
		};

		run(&mut out, args)?;

		assert_eq!(
			String::from_utf8(out).unwrap(),
			"ありがとうございます! いただきます!\n"
		);

		Ok(())
	}

	#[test]
	fn test_output_katakana() -> Result {
		let mut out = Vec::new();
		let args = Args {
			katakana: true,
			words: Vec::from(["arigatougozaimasu!".into(), "itadakimasu!".into()]),
			..Default::default()
		};

		run(&mut out, args)?;

		assert_eq!(
			String::from_utf8(out).unwrap(),
			"アリガトウゴザイマス! イタダキマス!\n"
		);

		Ok(())
	}
}
