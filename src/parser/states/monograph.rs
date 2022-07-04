use crate::{
	config::KanaTable,
	parser::{
		machine::{Next, NextState, Previous},
		util,
	},
};

use super::{Choonpu, Digraph, LongDigraph, Nasal};

#[derive(Debug, PartialEq)]
pub struct Monograph<'a>(pub &'a str);

impl<'a> Next<'a> for Monograph<'a> {
	const SIZE: usize = 2;

	fn next(self, table: &KanaTable<'a>) -> (Option<&'a str>, NextState<'a>) {
		let word = self.0;

		if util::utf8_word_count(word) < Self::SIZE {
			return (None, Nasal::prev(self).into());
		}

		let query = util::utf8_word_slice_until(&word, Self::SIZE);

		match table.syllabograms.get(query) {
			None => (None, Nasal::prev(self).into()),
			Some(s) => (
				Some(s),
				if table.graphemes.choonpu.is_some() {
					Choonpu::prev(self).into()
				} else {
					LongDigraph::prev(self).into()
				},
			),
		}
	}
}

impl<'a> Previous<'a, Digraph<'a>> for Monograph<'a> {
	fn prev(state: Digraph<'a>) -> Self {
		Self(state.0)
	}
}

#[cfg(test)]
mod tests {
	use std::collections::{HashMap, HashSet};

	use crate::config::{Grapheme, Graphemes};

	use super::*;

	#[test]
	fn test_small_word() {
		let current = Monograph("あ");
		let table = KanaTable::default();
		let (result, next) = current.next(&table);

		assert_eq!(result, None);
		assert_eq!(next, Nasal("あ").into());
	}

	#[test]
	fn test_no_match() {
		let current = Monograph("alo");
		let table = KanaTable::default();
		let (result, next) = current.next(&table);

		assert_eq!(result, None);
		assert_eq!(next, Nasal("alo").into());
	}

	#[test]
	fn test_regular_match() {
		let current = Monograph("test");
		let table = KanaTable {
			syllabograms: {
				let mut m = HashMap::new();
				m.insert("te", "@");
				m
			},
			..Default::default()
		};
		let (result, next) = current.next(&table);

		assert_eq!(result, Some("@"));
		assert_eq!(next, LongDigraph("st").into());
	}

	#[test]
	fn test_match_with_choonpu() {
		let current = Monograph("oii");
		let table = KanaTable {
			syllabograms: {
				let mut m = HashMap::new();
				m.insert("oi", "@");
				m
			},
			graphemes: Graphemes {
				choonpu: Some(Grapheme {
					matches: {
						let mut s = HashSet::new();
						s.insert("ii");
						s
					},
					graph: "!",
				}),
				..Default::default()
			},
			..Default::default()
		};
		let (result, next) = current.next(&table);

		assert_eq!(result, Some("@"));
		assert_eq!(next, Choonpu("ii", false).into());
	}

	#[test]
	fn test_prev_digraph() {
		assert_eq!(Monograph::prev(Digraph("testing")), Monograph("testing"));
	}
}
