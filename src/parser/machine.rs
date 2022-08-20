use std::collections::HashMap;

use crate::config::KanaTable;

use super::states::{
	Choonpu, KanaToggle, Sukuon, Syllabogram, LONG_SIZE, MEDIUM_SIZE, SHORT_SIZE, TINY_SIZE,
};

pub type NextState<'a> = Option<State<'a>>;

#[derive(Debug, PartialEq)]
pub enum State<'a> {
	Long(Syllabogram<'a, LONG_SIZE>),
	Medium(Syllabogram<'a, MEDIUM_SIZE>),
	Short(Syllabogram<'a, SHORT_SIZE>),
	Tiny(Syllabogram<'a, TINY_SIZE>),
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
				State::Long(s) => s.next(table),
				State::Medium(s) => s.next(table),
				State::Short(s) => s.next(table),
				State::Tiny(s) => s.next(table),
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
	fn next(self, table: &KanaTable<'a>) -> (Option<&'a str>, NextState<'a>);
}

pub trait Previous<'a, T>
where
	T: Next<'a>,
{
	fn prev(state: T) -> Self;
}

impl<'a> From<Syllabogram<'a, LONG_SIZE>> for NextState<'a> {
	fn from(state: Syllabogram<'a, LONG_SIZE>) -> Self {
		Some(State::Long(state))
	}
}

impl<'a> From<Syllabogram<'a, MEDIUM_SIZE>> for NextState<'a> {
	fn from(state: Syllabogram<'a, MEDIUM_SIZE>) -> Self {
		Some(State::Medium(state))
	}
}

impl<'a> From<Syllabogram<'a, SHORT_SIZE>> for NextState<'a> {
	fn from(state: Syllabogram<'a, SHORT_SIZE>) -> Self {
		Some(State::Short(state))
	}
}

impl<'a> From<Syllabogram<'a, TINY_SIZE>> for NextState<'a> {
	fn from(state: Syllabogram<'a, TINY_SIZE>) -> Self {
		Some(State::Tiny(state))
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

		let tables = HashMap::from([(false, &hiragana), (true, &katakana)]);

		// Test against all hiragana syllabograms.
		for (input, want) in &hiragana.syllabograms {
			let result: Result<(), ()> = Machine::start(&tables, (false, None), input, |result| {
				assert_eq!(result, *want);
				Ok(())
			});

			assert!(result.is_ok());
		}

		// Test against real cases.
		let word_table = HashMap::from([
			("chottomatte", "ちょっとまって"),
			(
				"nikugazenzensukijaarimasen",
				"にくがぜんぜんすきじゃありません",
			),
			("wwwwwww", "wwwwwww"),
			("日本", "日本"),
			("nihon", "にほん"),
			("12jinitabemasu!", "12じにたべます!"),
			("123 GO!", "123 GO!"),
		]);

		for (input, want) in word_table {
			let result: Result<(), ()> = Machine::start(&tables, (false, None), input, |result| {
				assert_eq!(result, want);
				Ok(())
			});

			assert!(result.is_ok());
		}

		// Test against real cases with toggling.
		let word_table = HashMap::from([("watashiha@gaburieru@desu", "わたしはガブリエルです")]);

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

		let tables = HashMap::from([(false, &hiragana), (true, &katakana)]);

		// Test against all katakana syllabograms.
		for (input, want) in &katakana.syllabograms {
			let result: Result<(), ()> = Machine::start(&tables, (true, None), input, |result| {
				assert_eq!(result, *want);
				Ok(())
			});

			assert!(result.is_ok());
		}

		// Test against real cases.
		let word_table = HashMap::from([
			("oomen", "オーメン"),
			("tsyuu", "ツュー"),
			("suupaamario", "スーパーマリオ"),
			("pureisuteeshon", "プレイステーション"),
			("monkii D. rufi", "モンキー D. ルフィ"),
			("wwwwwww", "wwwwwww"),
			("supagetti", "スパゲッティ"),
			("日本", "日本"),
			("123 GO!", "123 GO!"),
		]);

		for (input, want) in word_table {
			let result: Result<(), ()> = Machine::start(&tables, (true, None), input, |result| {
				assert_eq!(result, want);
				Ok(())
			});

			assert!(result.is_ok());
		}

		// Test against real cases with toggling.
		let word_table = HashMap::from([("watashiha@gaburieru@desu", "ワタシハがぶりえるデス")]);

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
