use super::glyphs::{
	hiragana::{
		SOKUON_GRAPH as HIRAGANA_SOKUON_GRAPH, SOKUON_MATCHES as HIRAGANA_SOKUON_MATCHES,
		SYLLABARY as HIRAGANA,
	},
	katakana::{
		CHOUONPU_GRAPH, CHOUONPU_MATCHES, EXTENDED_SYLLABARY as EXTENDED_KATAKANA,
		SMALL_VOWELS as KATAKANA_SMALL_VOWELS, SOKUON_GRAPH as KATAKANA_SOKUON_GRAPH,
		SOKUON_MATCHES as KATAKANA_SOKUON_MATCHES, SYLLABARY as KATAKANA,
	},
};

pub enum Kana {
	Hiragana,
	Katakana { extended: bool },
}

impl Default for Kana {
	/// Hiragana is the default.
	fn default() -> Self {
		Self::Hiragana
	}
}

impl Kana {
	pub fn get(&self, key: &str) -> Option<&'static str> {
		let key = &key.to_lowercase();

		match self {
			Self::Hiragana => (&HIRAGANA).get(key),
			Self::Katakana { extended } => if *extended {
				(&EXTENDED_KATAKANA).get(key)
			} else {
				None
			}
			.or_else(|| (&KATAKANA).get(key)),
		}
		.map(|s| *s)
	}

	pub fn sokuon(&self, key: &str) -> Option<&'static str> {
		let key = &key.to_lowercase();

		let matches = match self {
			Self::Hiragana => &HIRAGANA_SOKUON_MATCHES,
			Self::Katakana { .. } => &KATAKANA_SOKUON_MATCHES,
		};

		match matches.contains(key) {
			true => Some(self.sokuon_literal()),
			false => None,
		}
	}

	pub fn sokuon_literal(&self) -> &'static str {
		match self {
			Self::Hiragana => &HIRAGANA_SOKUON_GRAPH,
			Self::Katakana { .. } => &KATAKANA_SOKUON_GRAPH,
		}
	}

	pub fn chouonpu(&self, key: &str) -> Option<&'static str> {
		let key = &key.to_lowercase();

		let (matches, graph) = match self {
			Self::Hiragana => return None,
			Self::Katakana { .. } => (&CHOUONPU_MATCHES, &CHOUONPU_GRAPH),
		};

		matches.get_key(key).map(|_| *graph)
	}

	pub fn small_vowel(&self, key: &str) -> Option<&'static str> {
		let key = &key.to_lowercase();

		match self {
			Self::Katakana { .. } => (&KATAKANA_SMALL_VOWELS).get(key).map(|vowel| *vowel),
			_ => None,
		}
	}
}
