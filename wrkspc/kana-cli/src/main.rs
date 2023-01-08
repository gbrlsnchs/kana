use std::io::{self, BufRead, Result as IoResult, Write};

use clap::Parser;

use cli::Kana;
use kana::{CharFeature, Config, SpecialChars};

mod cli;

fn main() -> IoResult<()> {
	let app = Kana::parse();
	let cfg = Config {
		start_with_katakana: app.katakana,
		extended_katakana: app.extended_katakana,
		parse_punctuation: app.with_punctuation,
		special_chars: {
			let mut s = SpecialChars::new();
			app.kana_toggle
				.and_then(|c| s.insert(CharFeature::KanaToggle, c));
			app.raw_text_toggle
				.and_then(|c| s.insert(CharFeature::RawTextToggle, c));
			app.prolongation_reset_char
				.and_then(|c| s.insert(CharFeature::ResetProlongation, c));
			app.small_vowel_char
				.and_then(|c| s.insert(CharFeature::SmallVowel, c));
			app.virtual_stop_char
				.and_then(|c| s.insert(CharFeature::VirtualStop, c));
			s
		},
	};

	let mut stdout = io::stdout().lock();

	if app.interactive {
		for line in io::stdin().lock().lines() {
			let output = kana::transliterate(&line.unwrap(), &cfg);

			if !output.is_empty() {
				writeln!(&mut stdout, "{}", output)?;
			}
		}
	} else {
		let romaji: String = app.input.unwrap_or_default().join(" ");
		let output = kana::transliterate(&romaji, &cfg);

		if !output.is_empty() {
			writeln!(&mut stdout, "{}", output)?;
		}
	}

	Ok(())
}
