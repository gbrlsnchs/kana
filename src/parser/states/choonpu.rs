use crate::{
	config::KanaTable,
	parser::{
		machine::{self, Next, Previous},
		util,
	},
};

use super::{KanaToggle, Syllabogram, LONG_SIZE, MEDIUM_SIZE, SHORT_SIZE, TINY_SIZE};

#[derive(Debug, PartialEq)]
pub struct Choonpu<'a>(pub &'a str, pub Option<char>, pub bool);

impl<'a> Next<'a> for Choonpu<'a> {
	const SIZE: usize = 2;

	fn next(mut self, table: &KanaTable<'a>) -> (Option<&'a str>, machine::NextState<'a>) {
		let word = self.0;

		if util::utf8_word_count(word) < Self::SIZE {
			return (None, Syllabogram::<'a, TINY_SIZE>::prev(self).into());
		}

		let query = util::utf8_word_slice_until(word, Self::SIZE);

		if let Some(choonpu) = &table.graphemes.choonpu {
			if choonpu.matches.contains(query) {
				self.2 = true;

				return (Some(choonpu.graph), KanaToggle::prev(self).into());
			}
		}

		(None, KanaToggle::prev(self).into())
	}
}

impl<'a> Previous<'a, Syllabogram<'a, LONG_SIZE>> for Choonpu<'a> {
	fn prev(state: Syllabogram<'a, LONG_SIZE>) -> Self {
		Self(
			util::utf8_word_slice_from(state.0, Syllabogram::<'a, LONG_SIZE>::SIZE - 1),
			state.1,
			false,
		)
	}
}

impl<'a> Previous<'a, Syllabogram<'a, MEDIUM_SIZE>> for Choonpu<'a> {
	fn prev(state: Syllabogram<'a, MEDIUM_SIZE>) -> Self {
		Self(
			util::utf8_word_slice_from(state.0, Syllabogram::<'a, MEDIUM_SIZE>::SIZE - 1),
			state.1,
			false,
		)
	}
}

impl<'a> Previous<'a, Syllabogram<'a, SHORT_SIZE>> for Choonpu<'a> {
	fn prev(state: Syllabogram<'a, SHORT_SIZE>) -> Self {
		Self(
			util::utf8_word_slice_from(state.0, Syllabogram::<'a, SHORT_SIZE>::SIZE - 1),
			state.1,
			false,
		)
	}
}

impl<'a> Previous<'a, Syllabogram<'a, TINY_SIZE>> for Choonpu<'a> {
	fn prev(state: Syllabogram<'a, TINY_SIZE>) -> Self {
		Self(state.0, state.1, false)
	}
}

#[cfg(test)]
mod tests {
	use std::collections::{HashMap, HashSet};

	use crate::config::{Grapheme, Graphemes};

	use super::*;

	#[test]
	fn test_small_word<'a>() {
		let current = Choonpu("ツ", None, false);
		let table = KanaTable::default();
		let next = current.next(&table);

		assert_eq!((None, Syllabogram::<'a, TINY_SIZE>("", None).into()), next);
	}

	#[test]
	fn test_no_match() {
		let current = Choonpu("oomen", None, false);
		let table = KanaTable::default();
		let (result, next) = current.next(&table);

		assert_eq!(result, None);
		assert_eq!(next, KanaToggle("omen", None, false).into());
	}

	#[test]
	fn test_regular_match() {
		let current = Choonpu("oomen", None, false);
		let table = KanaTable {
			syllabograms: HashMap::from([("omen", "@")]),
			graphemes: Graphemes {
				choonpu: Some(Grapheme {
					matches: HashSet::from(["oo"]),
					graph: "~",
				}),
				..Default::default()
			},
		};
		let (result, next) = current.next(&table);

		assert_eq!(result, Some("~"));
		assert_eq!(next, KanaToggle("men", None, false).into());
	}

	#[test]
	fn test_prev_long_digraph<'a>() {
		assert_eq!(
			Choonpu::prev(Syllabogram::<'a, LONG_SIZE>("testing", None)),
			Choonpu("ting", None, false),
		);
		assert_eq!(
			Choonpu::prev(Syllabogram::<'a, LONG_SIZE>("testing", Some('@'))),
			Choonpu("ting", Some('@'), false),
		);
	}

	#[test]
	fn test_prev_digraph<'a>() {
		assert_eq!(
			Choonpu::prev(Syllabogram::<'a, MEDIUM_SIZE>("testing", None)),
			Choonpu("sting", None, false)
		);
		assert_eq!(
			Choonpu::prev(Syllabogram::<'a, MEDIUM_SIZE>("testing", Some('@'))),
			Choonpu("sting", Some('@'), false)
		);
	}

	#[test]
	fn test_prev_monograph<'a>() {
		assert_eq!(
			Choonpu::prev(Syllabogram::<'a, SHORT_SIZE>("testing", None)),
			Choonpu("esting", None, false)
		);
		assert_eq!(
			Choonpu::prev(Syllabogram::<'a, SHORT_SIZE>("testing", Some('@'))),
			Choonpu("esting", Some('@'), false)
		);
	}

	#[test]
	fn test_prev_nasal<'a>() {
		assert_eq!(
			Choonpu::prev(Syllabogram::<'a, TINY_SIZE>("testing", None)),
			Choonpu("testing", None, false)
		);
		assert_eq!(
			Choonpu::prev(Syllabogram::<'a, TINY_SIZE>("testing", Some('@'))),
			Choonpu("testing", Some('@'), false)
		);
	}
}
