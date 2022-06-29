use std::io::Result as IoResult;

use crate::parser::{Computation, Parser, Section};
use crate::spec::Spec;

#[derive(Debug, PartialEq)]
pub struct State<'a> {
	pub section: Section,
	pub word: &'a str,
}

impl<'a> State<'a> {
	pub fn init<F>(spec: &Spec, word: &'a str, mut handle: F) -> IoResult<()>
	where
		F: FnMut(String) -> IoResult<()>,
	{
		let mut state = State {
			section: Section::default(),
			word,
		};

		let mut computed = String::with_capacity(word.len() * 2);

		while let Some((result, next_state)) = state.compute(&spec) {
			if let Some(symbol) = result {
				computed.push_str(symbol);
			}

			state = next_state;
		}

		handle(computed)
	}

	fn compute(self, spec: &'a Spec) -> Option<Computation<'a>> {
		let Self { word, section } = self;

		match section {
			Section::Digraph => Parser::<{ Section::Digraph as usize }>::next(spec, word),
			Section::Monograph => Parser::<{ Section::Monograph as usize }>::next(spec, word),
			Section::Nasal => Parser::<{ Section::Nasal as usize }>::next(spec, word),
			Section::Sukuon => Parser::<{ Section::Sukuon as usize }>::next(spec, word),
			Section::Choonpu => Parser::<{ Section::Choonpu as usize }>::next(spec, word),
			Section::LongDigraph => Parser::<{ Section::LongDigraph as usize }>::next(spec, word),
		}
	}
}

#[cfg(test)]
mod tests {
	use std::collections::HashMap;

	use crate::run::load_kanas;

	use super::*;

	#[test]
	fn test_hiragana() -> IoResult<()> {
		let word_table = {
			let mut v = HashMap::new();
			v.insert("chottomatte", "ちょっとまって");
			v.insert(
				"nikugazenzensukijaarimasen",
				"にくがぜんぜんすきじゃありません",
			);
			v.insert("wwwwwww", "wwwwwww");
			v.insert("日本", "日本");
			v.insert("nihon", "にほん");
			v
		};

		let (hiragana, _) = load_kanas();
		let spec = toml::de::from_str(hiragana)?;

		for (input, want) in word_table {
			let result = State::init(&spec, input, |result| {
				assert_eq!(result, want);
				Ok(())
			});

			assert!(result.is_ok());
		}

		Ok(())
	}

	#[test]
	fn test_katakana() -> IoResult<()> {
		let word_table = {
			let mut v = HashMap::new();
			v.insert("suupaamario", "スーパーマリオ");
			v.insert("pureisuteeshon", "プレイステーション");
			v.insert("monkii D. ruufii", "モンキー D. ルーフィー");
			v.insert("wwwwwww", "wwwwwww");
			v.insert("supagetti", "スパゲッティ");
			v.insert("日本", "日本");
			v
		};

		let (_, katakana) = load_kanas();
		let spec = toml::de::from_str(katakana)?;

		for (input, want) in word_table {
			let result = State::init(&spec, input, |result| {
				assert_eq!(result, want);
				Ok(())
			});

			assert!(result.is_ok());
		}

		Ok(())
	}
}
