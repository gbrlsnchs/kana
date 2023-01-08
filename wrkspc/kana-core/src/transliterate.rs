use std::collections::HashMap;

use crate::parser::{fsm::State, input::Input};

#[derive(Eq, Hash, PartialEq)]
pub enum CharFeature {
	KanaToggle,
	RawTextToggle,
	Reset,
	SmallVowel,
	VirtualSokuon,
}

pub type SpecialChars = HashMap<CharFeature, char>;

#[derive(Default)]
pub struct Config {
	pub start_with_katakana: bool,
	pub extended_katakana: bool,
	pub parse_punctuation: bool,
	pub special_chars: SpecialChars,
}

/// Transforms romaji input into hiragana/katakana.
pub fn transliterate(romaji: &str, cfg: &Config) -> String {
	let mut state = State::Init;
	let mut input = Input::new(romaji, cfg);

	let mut result = String::with_capacity(romaji.len() * 4);

	while let Some((output, next_input, next_state)) = state.next(input) {
		result.push_str(output);

		state = next_state;
		input = next_input;
	}

	result
}
