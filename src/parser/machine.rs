use std::collections::HashMap;

use crate::config::KanaTable;

use super::states::{Choonpu, Digraph, KanaToggle, LongDigraph, Monograph, Nasal, Sukuon};

pub type NextState<'a> = Option<State<'a>>;

#[derive(Debug, PartialEq)]
pub enum State<'a> {
	LongDigraph(LongDigraph<'a>),
	Digraph(Digraph<'a>),
	Monograph(Monograph<'a>),
	Nasal(Nasal<'a>),
	Sukuon(Sukuon<'a>),
	Choonpu(Choonpu<'a>),
	KanaToggle(KanaToggle<'a>),
}

pub struct Machine;

impl Machine {
	pub fn start<F, R>(
		tables: &HashMap<bool, &KanaTable>,
		toggles: (bool, Option<char>),
		word: &str,
		mut handle: F,
	) -> R
	where
		F: FnMut(String) -> R,
	{
		let (mut katakana, toggle_char) = toggles;
		let mut table = tables.get(&katakana).unwrap();
		let mut state = State::KanaToggle(KanaToggle(word, toggle_char, false));
		let mut result = String::with_capacity(word.len() * 2);

		loop {
			let (s, next_state) = match state {
				State::LongDigraph(s) => s.next(table),
				State::Digraph(s) => s.next(table),
				State::Monograph(s) => s.next(table),
				State::Nasal(s) => s.next(table),
				State::Sukuon(s) => s.next(table),
				State::Choonpu(s) => s.next(table),
				State::KanaToggle(s) => {
					let KanaToggle(_, _, matches) = s;

					if matches {
						katakana = !katakana;
						table = tables.get(&katakana).unwrap();
					}

					s.next(table)
				}
			};

			if let Some(s) = s {
				result.push_str(s);
			}

			state = match next_state {
				None => break,
				Some(s) => s,
			};
		}

		handle(result)
	}
}

pub trait Next<'a> {
	const SIZE: usize;

	fn next(self, table: &KanaTable<'a>) -> (Option<&'a str>, NextState<'a>);
}

pub trait Previous<'a, T>
where
	T: Next<'a>,
{
	fn prev(state: T) -> Self;
}

impl<'a> From<LongDigraph<'a>> for NextState<'a> {
	fn from(state: LongDigraph<'a>) -> Self {
		Some(State::LongDigraph(state))
	}
}

impl<'a> From<Digraph<'a>> for NextState<'a> {
	fn from(state: Digraph<'a>) -> Self {
		Some(State::Digraph(state))
	}
}

impl<'a> From<Monograph<'a>> for NextState<'a> {
	fn from(state: Monograph<'a>) -> Self {
		Some(State::Monograph(state))
	}
}

impl<'a> From<Nasal<'a>> for NextState<'a> {
	fn from(state: Nasal<'a>) -> Self {
		Some(State::Nasal(state))
	}
}

impl<'a> From<Sukuon<'a>> for NextState<'a> {
	fn from(state: Sukuon<'a>) -> Self {
		Some(State::Sukuon(state))
	}
}

impl<'a> From<Choonpu<'a>> for NextState<'a> {
	fn from(state: Choonpu<'a>) -> Self {
		Some(State::Choonpu(state))
	}
}

impl<'a> From<KanaToggle<'a>> for NextState<'a> {
	fn from(state: KanaToggle<'a>) -> Self {
		Some(State::KanaToggle(state))
	}
}

#[cfg(test)]
mod tests {
	use std::{collections::HashMap, io::Result as IoResult};

	use crate::run::load_kanas;

	use super::*;

	#[test]
	fn test_hiragana() -> IoResult<()> {
		let (hiragana, katakana) = load_kanas();
		let hiragana: KanaTable = toml::de::from_str(hiragana)?;
		let katakana: KanaTable = toml::de::from_str(katakana)?;

		let tables = {
			let mut m = HashMap::new();
			m.insert(false, &hiragana);
			m.insert(true, &katakana);
			m
		};

		// Test against all hiragana syllabograms.
		for (input, want) in &hiragana.syllabograms {
			let result: Result<(), ()> = Machine::start(&tables, (false, None), input, |result| {
				assert_eq!(result, *want);
				Ok(())
			});

			assert!(result.is_ok());
		}

		// Test against real cases.
		let word_table = {
			let mut m = HashMap::new();
			m.insert("chottomatte", "ちょっとまって");
			m.insert(
				"nikugazenzensukijaarimasen",
				"にくがぜんぜんすきじゃありません",
			);
			m.insert("wwwwwww", "wwwwwww");
			m.insert("日本", "日本");
			m.insert("nihon", "にほん");
			m.insert("12jinitabemasu!", "12じにたべます!");
			m.insert("123 GO!", "123 GO!");
			m
		};

		for (input, want) in word_table {
			let result: Result<(), ()> = Machine::start(&tables, (false, None), input, |result| {
				assert_eq!(result, want);
				Ok(())
			});

			assert!(result.is_ok());
		}

		// Test against real cases with toggling.
		let word_table = {
			let mut m = HashMap::new();
			m.insert("watashiha@gaburieru@desu", "わたしはガブリエルです");
			m
		};

		for (input, want) in word_table {
			let result: Result<(), ()> =
				Machine::start(&tables, (false, Some('@')), input, |result| {
					assert_eq!(result, want);
					Ok(())
				});

			assert!(result.is_ok());
		}

		Ok(())
	}

	#[test]
	fn test_katakana() -> IoResult<()> {
		let (hiragana, katakana) = load_kanas();
		let hiragana: KanaTable = toml::de::from_str(hiragana)?;
		let katakana: KanaTable = toml::de::from_str(katakana)?;

		let tables = {
			let mut m = HashMap::new();
			m.insert(false, &hiragana);
			m.insert(true, &katakana);
			m
		};

		// Test against all katakana syllabograms.
		for (input, want) in &katakana.syllabograms {
			let result: Result<(), ()> = Machine::start(&tables, (true, None), input, |result| {
				assert_eq!(result, *want);
				Ok(())
			});

			assert!(result.is_ok());
		}

		// Test against real cases.
		let word_table = {
			let mut m = HashMap::new();
			m.insert("oomen", "オーメン");
			m.insert("tsyuu", "ツュー");
			m.insert("suupaamario", "スーパーマリオ");
			m.insert("pureisuteeshon", "プレイステーション");
			m.insert("monkii D. rufi", "モンキー D. ルフィ");
			m.insert("wwwwwww", "wwwwwww");
			m.insert("supagetti", "スパゲッティ");
			m.insert("日本", "日本");
			m.insert("123 GO!", "123 GO!");
			m
		};

		for (input, want) in word_table {
			let result: Result<(), ()> = Machine::start(&tables, (true, None), input, |result| {
				assert_eq!(result, want);
				Ok(())
			});

			assert!(result.is_ok());
		}

		// Test against real cases with toggling.
		let word_table = {
			let mut m = HashMap::new();
			m.insert("watashiha@gaburieru@desu", "ワタシハがぶりえるデス");
			m
		};

		for (input, want) in word_table {
			let result: Result<(), ()> =
				Machine::start(&tables, (true, Some('@')), input, |result| {
					assert_eq!(result, want);
					Ok(())
				});

			assert!(result.is_ok());
		}

		Ok(())
	}
}
