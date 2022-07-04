use crate::{
	config::KanaTable,
	parser::{
		machine::{self, Next, Previous},
		util,
	},
};

use super::{Digraph, LongDigraph, Monograph, Nasal};

#[derive(Debug, PartialEq)]
pub struct Choonpu<'a>(pub &'a str, pub bool);

impl<'a> Next<'a> for Choonpu<'a> {
	const SIZE: usize = 2;

	fn next(mut self, table: &KanaTable<'a>) -> (Option<&'a str>, machine::NextState<'a>) {
		let word = self.0;

		if util::utf8_word_count(word) < Self::SIZE {
			return (None, Nasal::prev(self).into());
		}

		let query = util::utf8_word_slice_until(word, Self::SIZE);

		if let Some(choonpu) = &table.graphemes.choonpu {
			if choonpu.matches.contains(query) {
				self.1 = true;

				return (Some(choonpu.graph), LongDigraph::prev(self).into());
			}
		}

		(None, LongDigraph::prev(self).into())
	}
}

impl<'a> Previous<'a, LongDigraph<'a>> for Choonpu<'a> {
	fn prev(state: LongDigraph<'a>) -> Self {
		Self(
			util::utf8_word_slice_from(state.0, LongDigraph::SIZE - 1),
			false,
		)
	}
}

impl<'a> Previous<'a, Digraph<'a>> for Choonpu<'a> {
	fn prev(state: Digraph<'a>) -> Self {
		Self(
			util::utf8_word_slice_from(state.0, Digraph::SIZE - 1),
			false,
		)
	}
}

impl<'a> Previous<'a, Monograph<'a>> for Choonpu<'a> {
	fn prev(state: Monograph<'a>) -> Self {
		Self(
			util::utf8_word_slice_from(state.0, Monograph::SIZE - 1),
			false,
		)
	}
}

impl<'a> Previous<'a, Nasal<'a>> for Choonpu<'a> {
	fn prev(state: Nasal<'a>) -> Self {
		Self(state.0, false)
	}
}

#[cfg(test)]
mod tests {
	use std::collections::{HashMap, HashSet};

	use crate::config::{Grapheme, Graphemes};

	use super::*;

	#[test]
	fn test_small_word() {
		let current = Choonpu("ツ", false);
		let table = KanaTable::default();
		let next = current.next(&table);

		assert_eq!((None, Nasal("").into()), next);
	}

	#[test]
	fn test_no_match() {
		let current = Choonpu("oomen", false);
		let table = KanaTable::default();
		let (result, next) = current.next(&table);

		assert_eq!(result, None);
		assert_eq!(next, LongDigraph("omen").into());
	}

	#[test]
	fn test_regular_match() {
		let current = Choonpu("oomen", false);
		let table = KanaTable {
			syllabograms: {
				let mut m = HashMap::new();
				m.insert("omen", "@");
				m
			},
			graphemes: Graphemes {
				choonpu: Some(Grapheme {
					matches: {
						let mut s = HashSet::new();
						s.insert("oo");
						s
					},
					graph: "~",
				}),
				..Default::default()
			},
		};
		let (result, next) = current.next(&table);

		assert_eq!(result, Some("~"));
		assert_eq!(next, LongDigraph("men").into());
	}
}
