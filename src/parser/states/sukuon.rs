use crate::{
	config::KanaTable,
	parser::{
		machine::{Next, NextState, Previous},
		util,
	},
};

use super::{Digraph, LongDigraph, Nasal};

#[derive(Debug, PartialEq)]
pub struct Sukuon<'a>(pub &'a str);

impl<'a> Next<'a> for Sukuon<'a> {
	const SIZE: usize = 2;

	fn next(self, table: &KanaTable<'a>) -> (Option<&'a str>, NextState<'a>) {
		let word = self.0;

		if util::utf8_word_count(word) < Self::SIZE {
			return (None, Nasal::prev(self).into());
		}

		let query = util::utf8_word_slice_until(word, Self::SIZE);

		match table.graphemes.sukuon.matches.contains(query) {
			true => (
				Some(table.graphemes.sukuon.graph),
				LongDigraph::prev(self).into(),
			),
			false => (None, Digraph::prev(self).into()),
		}
	}
}

impl<'a> Previous<'a, LongDigraph<'a>> for Sukuon<'a> {
	fn prev(state: LongDigraph<'a>) -> Self {
		Sukuon(state.0)
	}
}

#[cfg(test)]
mod tests {
	use std::collections::{HashMap, HashSet};

	use crate::config::{Grapheme, Graphemes};

	use super::*;

	#[test]
	fn test_small_word() {
		let current = Sukuon("ツ");
		let table = KanaTable::default();
		let next = current.next(&table);

		assert_eq!((None, Nasal("ツ").into()), next);
	}

	#[test]
	fn test_no_match() {
		let current = Sukuon("tto");
		let table = KanaTable::default();
		let (result, next) = current.next(&table);

		// This state returns the original query if nothing is found, since it's essentially the
		// last parsing step, this way preserving untranslatable characters prev the original word.
		assert_eq!(result, None);
		assert_eq!(next, Digraph("tto").into());
	}

	#[test]
	fn test_regular_match() {
		let current = Sukuon("tto");
		let table = KanaTable {
			syllabograms: {
				let mut m = HashMap::new();
				m.insert("to", "@");
				m
			},
			graphemes: Graphemes {
				sukuon: Grapheme {
					matches: {
						let mut s = HashSet::new();
						s.insert("tt");
						s
					},
					graph: "+",
				},
				..Default::default()
			},
		};
		let (result, next) = current.next(&table);

		assert_eq!(result, Some("+"));
		assert_eq!(next, LongDigraph("to").into());
	}
}