use crate::{
	config::KanaTable,
	parser::{
		machine::{Next, NextState, Previous},
		util,
	},
};

use super::{monograph::Monograph, sukuon::Sukuon, Choonpu, LongDigraph};

#[derive(Debug, PartialEq)]
pub struct Digraph<'a>(pub &'a str);

impl<'a> Next<'a> for Digraph<'a> {
	const SIZE: usize = 3;

	fn next(self, table: &KanaTable<'a>) -> (Option<&'a str>, NextState<'a>) {
		let word = self.0;

		if util::utf8_word_count(word) < Self::SIZE {
			return (None, Monograph::prev(self).into());
		}

		let query = util::utf8_word_slice_until(word, Self::SIZE);

		match table.syllabograms.get(query) {
			None => (None, Monograph::prev(self).into()),
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

impl<'a> Previous<'a, Sukuon<'a>> for Digraph<'a> {
	fn prev(state: Sukuon<'a>) -> Self {
		Digraph(state.0)
	}
}

#[cfg(test)]
mod tests {
	use std::collections::{HashMap, HashSet};

	use crate::config::{Grapheme, Graphemes};

	use super::*;

	#[test]
	fn test_small_word() {
		let current = Digraph("oi");
		let table = KanaTable::default();
		let (result, next) = current.next(&table);

		assert_eq!(result, None);
		assert_eq!(next, Monograph("oi").into());
	}

	#[test]
	fn test_no_match() {
		let current = Digraph("alo");
		let table = KanaTable::default();
		let (result, next) = current.next(&table);

		assert_eq!(result, None);
		assert_eq!(next, Monograph("alo").into());
	}

	#[test]
	fn test_regular_match() {
		let current = Digraph("foobar");
		let table = KanaTable {
			syllabograms: {
				let mut m = HashMap::new();
				m.insert("foo", "@");
				m
			},
			..Default::default()
		};
		let (result, next) = current.next(&table);

		assert_eq!(result, Some("@"));
		assert_eq!(next, LongDigraph("bar").into());
	}

	#[test]
	fn test_match_with_choonpu() {
		let current = Digraph("olaa");
		let table = KanaTable {
			syllabograms: {
				let mut m = HashMap::new();
				m.insert("ola", "@");
				m
			},
			graphemes: Graphemes {
				choonpu: Some(Grapheme {
					matches: {
						let mut s = HashSet::new();
						s.insert("aa");
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
		assert_eq!(next, Choonpu("aa", false).into());
	}

	#[test]
	fn test_prev_sukuon() {
		assert_eq!(Digraph::prev(Sukuon("testing")), Digraph("testing"));
	}
}
