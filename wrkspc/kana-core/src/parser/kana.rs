use super::glyphs::{
	hiragana::{
		SOKUON_GRAPH as HIRAGANA_SOKUON_GRAPH, SOKUON_MATCHES as HIRAGANA_SOKUON_MATCHES,
		SYLLABARY as HIRAGANA,
	},
	katakana::{
		CHOONPU_GRAPH, CHOONPU_MATCHES, EXTENDED_SYLLABARY as EXTENDED_KATAKANA,
		SOKUON_GRAPH as KATAKANA_SOKUON_GRAPH, SOKUON_MATCHES as KATAKANA_SOKUON_MATCHES,
		SYLLABARY as KATAKANA,
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

		let (matches, graph) = match self {
			Self::Hiragana => (&HIRAGANA_SOKUON_MATCHES, &HIRAGANA_SOKUON_GRAPH),
			Self::Katakana { .. } => (&KATAKANA_SOKUON_MATCHES, &KATAKANA_SOKUON_GRAPH),
		};

		matches.get_key(key).map(|_| *graph)
	}

	pub fn choonpu(&self, key: &str) -> Option<&'static str> {
		let key = &key.to_lowercase();

		let (matches, graph) = match self {
			Self::Hiragana => return None,
			Self::Katakana { .. } => (&CHOONPU_MATCHES, &CHOONPU_GRAPH),
		};

		matches.get_key(key).map(|_| *graph)
	}
}
