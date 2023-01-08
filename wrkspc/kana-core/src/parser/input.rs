use crate::transliterate::{Config, SpecialChars};

use super::{
	glyphs::punctuation::{DOUBLE_QUOTES, SINGLE_QUOTES},
	kana::Kana,
	switch::Switch,
};

pub struct Input<'a> {
	pub romaji: &'a str,
	pub special_chars: &'a SpecialChars,
	pub kanas: Switch<Kana>,
	pub punctuation: Option<Punctuation>,
}

impl<'a> Input<'a> {
	pub fn new(romaji: &'a str, cfg: &'a Config) -> Self {
		Input {
			romaji,
			special_chars: &cfg.special_chars,
			punctuation: if cfg.parse_punctuation {
				Some(Punctuation {
					single_quotes: Switch::new(SINGLE_QUOTES),
					double_quotes: Switch::new(DOUBLE_QUOTES),
				})
			} else {
				None
			},
			kanas: {
				let mut sw = Switch::new([
					Kana::Hiragana,
					Kana::Katakana {
						extended: cfg.extended_katakana,
					},
				]);

				if cfg.start_with_katakana {
					sw.toggle();
				}

				sw
			},
		}
	}
}

pub struct Punctuation {
	pub single_quotes: Switch<&'static str>,
	pub double_quotes: Switch<&'static str>,
}
