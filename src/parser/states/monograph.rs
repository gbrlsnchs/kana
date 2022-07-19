use crate::{
	config::KanaTable,
	parser::{
		machine::{Next, NextState, Previous},
		util,
	},
};

use super::{Choonpu, Digraph, KanaToggle, Nasal};

#[derive(Debug, PartialEq)]
pub struct Monograph<'a>(pub &'a str, pub Option<char>);

impl<'a> Next<'a> for Monograph<'a> {
	const SIZE: usize = 2;

	fn next(self, table: &KanaTable<'a>) -> (Option<&'a str>, NextState<'a>) {
		let word = self.0;

		if util::utf8_word_count(word) < Self::SIZE {
			return (None, Nasal::prev(self).into());
		}

		let query = util::utf8_word_slice_until(word, Self::SIZE);

		match table.syllabograms.get(query) {
			None => (None, Nasal::prev(self).into()),
			Some(s) => (
				Some(s),
				if table.graphemes.choonpu.is_some() {
					Choonpu::prev(self).into()
				} else {
					KanaToggle::prev(self).into()
				},
			),
		}
	}
}

impl<'a> Previous<'a, Digraph<'a>> for Monograph<'a> {
	fn prev(state: Digraph<'a>) -> Self {
		Self(state.0, state.1)
	}
}

#[cfg(test)]
mod tests {
	use std::collections::{HashMap, HashSet};

	use crate::config::{Grapheme, Graphemes};

	use super::*;

	#[test]
	fn test_small_word() {
		let current = Monograph("あ", None);
		let table = KanaTable::default();
		let (result, next) = current.next(&table);

		assert_eq!(result, None);
		assert_eq!(next, Nasal("あ", None).into());
	}

	#[test]
	fn test_no_match() {
		let current = Monograph("alo", None);
		let table = KanaTable::default();
		let (result, next) = current.next(&table);

		assert_eq!(result, None);
		assert_eq!(next, Nasal("alo", None).into());
	}

	#[test]
	fn test_regular_match() {
		let current = Monograph("test", None);
		let table = KanaTable {
			syllabograms: HashMap::from([("te", "@")]),
			..Default::default()
		};
		let (result, next) = current.next(&table);

		assert_eq!(result, Some("@"));
		assert_eq!(next, KanaToggle("st", None, false).into());
	}

	#[test]
	fn test_match_with_choonpu() {
		let current = Monograph("oii", None);
		let table = KanaTable {
			syllabograms: HashMap::from([("oi", "@")]),
			graphemes: Graphemes {
				choonpu: Some(Grapheme {
					matches: HashSet::from(["ii"]),
					graph: "!",
				}),
				..Default::default()
			},
			..Default::default()
		};
		let (result, next) = current.next(&table);

		assert_eq!(result, Some("@"));
		assert_eq!(next, Choonpu("ii", None, false).into());
	}

	#[test]
	fn test_prev_digraph() {
		assert_eq!(
			Monograph::prev(Digraph("testing", None)),
			Monograph("testing", None),
		);
		assert_eq!(
			Monograph::prev(Digraph("testing", Some('@'))),
			Monograph("testing", Some('@')),
		);
	}
}
