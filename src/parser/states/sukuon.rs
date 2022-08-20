use crate::{
	config::KanaTable,
	parser::{
		machine::{Next, NextState, Previous},
		util,
	},
};

use super::{KanaToggle, Syllabogram, LONG_SIZE, MEDIUM_SIZE, TINY_SIZE};

#[derive(Debug, PartialEq)]
pub struct Sukuon<'a>(pub &'a str, pub Option<char>);

impl<'a> Next<'a> for Sukuon<'a> {
	const SIZE: usize = 2;

	fn next(self, table: &KanaTable<'a>) -> (Option<&'a str>, NextState<'a>) {
		let word = self.0;

		if util::utf8_word_count(word) < Self::SIZE {
			return (None, Syllabogram::<'a, TINY_SIZE>::prev(self).into());
		}

		let query = util::utf8_word_slice_until(word, Self::SIZE);

		match table.graphemes.sukuon.matches.contains(query) {
			true => (
				Some(table.graphemes.sukuon.graph),
				KanaToggle::prev(self).into(),
			),
			false => (None, Syllabogram::<'a, MEDIUM_SIZE>::prev(self).into()),
		}
	}
}

impl<'a> Previous<'a, Syllabogram<'a, LONG_SIZE>> for Sukuon<'a> {
	fn prev(state: Syllabogram<'a, LONG_SIZE>) -> Self {
		Sukuon(state.0, state.1)
	}
}

#[cfg(test)]
mod tests {
	use std::collections::{HashMap, HashSet};

	use crate::config::{Grapheme, Graphemes};

	use super::*;

	#[test]
	fn test_small_word<'a>() {
		let current = Sukuon("ツ", None);
		let table = KanaTable::default();
		let next = current.next(&table);

		assert_eq!(
			(None, Syllabogram::<'a, TINY_SIZE>("ツ", None).into()),
			next
		);
	}

	#[test]
	fn test_no_match<'a>() {
		let current = Sukuon("tto", None);
		let table = KanaTable::default();
		let (result, next) = current.next(&table);

		// This state returns the original query if nothing is found, since it's essentially the
		// last parsing step, this way preserving untranslatable characters prev the original word.
		assert_eq!(result, None);
		assert_eq!(next, Syllabogram::<'a, MEDIUM_SIZE>("tto", None).into());
	}

	#[test]
	fn test_regular_match() {
		let current = Sukuon("tto", None);
		let table = KanaTable {
			syllabograms: HashMap::from([("to", "@")]),
			graphemes: Graphemes {
				sukuon: Grapheme {
					matches: HashSet::from(["tt"]),
					graph: "+",
				},
				..Default::default()
			},
		};
		let (result, next) = current.next(&table);

		assert_eq!(result, Some("+"));
		assert_eq!(next, KanaToggle("to", None, false).into());
	}

	#[test]
	fn test_prev_long_digraph<'a>() {
		assert_eq!(
			Sukuon::prev(Syllabogram::<'a, LONG_SIZE>("testing", None)),
			Sukuon("testing", None),
		);
		assert_eq!(
			Sukuon::prev(Syllabogram::<'a, LONG_SIZE>("testing", Some('@'))),
			Sukuon("testing", Some('@')),
		);
	}
}
