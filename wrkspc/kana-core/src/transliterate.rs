use std::collections::HashMap;

use crate::parser::{fsm::State, input::Input};

/// A feature represented by a character during parsing.
#[derive(Eq, Hash, PartialEq)]
pub enum CharFeature {
	/// Toggles between hiragana and katakana during the parsing.
	KanaToggle,
	/// Toggles between raw text and kanas during the parsing.
	RawTextToggle,
	/// Resets prolongations in katakana. Useful for repeating syllabograms instead of
	/// adding a chouonpu (ちょうおんぷ).
	ResetProlongation,
	/// Allows adding a small vowel. Useful for unconventional prolongations.
	SmallVowel,
	/// Allows adding a virtual stop, that is, a sokuon (そくおん). Probably only useful
	/// for loanwords.
	VirtualStop,
}

pub type SpecialChars = HashMap<CharFeature, char>;

/// Configuration for transliterating romaji text.
#[derive(Default)]
pub struct Config {
	/// Make the parser start transliterating to katakana instead of hiragana.
	pub start_with_katakana: bool,
	/// Use the extended version of the katakana syllabary.
	pub extended_katakana: bool,
	/// Consider punctuation marks when parsing the romaji text.
	pub parse_punctuation: bool,
	/// A map of special chars that can be used during the transliteration.
	pub special_chars: SpecialChars,
}

/// Transforms romaji input into hiragana/katakana. The romaji input is case-insensitve.
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
