use crate::{
	config::KanaTable,
	parser::{
		machine::{Next, NextState, Previous},
		util,
	},
};

use super::{Choonpu, Digraph, Monograph, Nasal, Sukuon};

#[derive(Debug, PartialEq)]
pub struct LongDigraph<'a>(pub &'a str);

impl<'a> Next<'a> for LongDigraph<'a> {
	const SIZE: usize = 4;

	fn next(self, table: &KanaTable<'a>) -> (Option<&'a str>, NextState<'a>) {
		let word = self.0;

		if util::utf8_word_count(word) < Self::SIZE {
			return (None, Sukuon::prev(self).into());
		}

		let query = util::utf8_word_slice_until(word, Self::SIZE);

		match table.syllabograms.get(query) {
			None => (None, Sukuon::prev(self).into()),
			Some(s) => (
				Some(s),
				if table.graphemes.choonpu.is_some() {
					Choonpu::prev(self).into()
				} else {
					Self::prev(self).into()
				},
			),
		}
	}
}

impl<'a> Previous<'a, LongDigraph<'a>> for LongDigraph<'a> {
	fn prev(state: LongDigraph<'a>) -> Self {
		Self(util::utf8_word_slice_from(state.0, Self::SIZE))
	}
}

impl<'a> Previous<'a, Digraph<'a>> for LongDigraph<'a> {
	fn prev(state: Digraph<'a>) -> Self {
		Self(util::utf8_word_slice_from(state.0, Digraph::SIZE))
	}
}

impl<'a> Previous<'a, Monograph<'a>> for LongDigraph<'a> {
	fn prev(state: Monograph<'a>) -> Self {
		Self(util::utf8_word_slice_from(state.0, Monograph::SIZE))
	}
}

impl<'a> Previous<'a, Nasal<'a>> for LongDigraph<'a> {
	fn prev(state: Nasal<'a>) -> Self {
		Self(util::utf8_word_slice_from(state.0, Nasal::SIZE))
	}
}

impl<'a> Previous<'a, Sukuon<'a>> for LongDigraph<'a> {
	fn prev(state: Sukuon<'a>) -> Self {
		Self(util::utf8_word_slice_from(state.0, Sukuon::SIZE - 1))
	}
}

impl<'a> Previous<'a, Choonpu<'a>> for LongDigraph<'a> {
	fn prev(state: Choonpu<'a>) -> Self {
		Self(util::utf8_word_slice_from(
			state.0,
			if state.1 {
				Choonpu::SIZE
			} else {
				Choonpu::SIZE - 1
			},
		))
	}
}

#[cfg(test)]
mod tests {
	use std::collections::{HashMap, HashSet};

	use crate::config::{Grapheme, Graphemes};

	use super::*;

	#[test]
	fn test_small_word() {
		let current = LongDigraph("テスト");
		let table = KanaTable::default();
		let (result, next) = current.next(&table);

		assert_eq!(result, None);
		assert_eq!(next, Sukuon("テスト").into());
	}

	#[test]
	fn test_no_match() {
		let current = LongDigraph("testing");
		let table = KanaTable::default();
		let (result, next) = current.next(&table);

		assert_eq!(result, None);
		assert_eq!(next, Sukuon("testing").into());
	}

	#[test]
	fn test_regular_match() {
		let current = LongDigraph("testing");
		let table = KanaTable {
			syllabograms: {
				let mut m = HashMap::new();
				m.insert("test", "@");
				m
			},
			..Default::default()
		};
		let (result, next) = current.next(&table);

		assert_eq!(result, Some("@"));
		assert_eq!(next, LongDigraph("ing").into());
	}

	#[test]
	fn test_match_with_choonpu() {
		let current = LongDigraph("yahoo");
		let table = KanaTable {
			syllabograms: {
				let mut m = HashMap::new();
				m.insert("yaho", "@");
				m
			},
			graphemes: Graphemes {
				choonpu: Some(Grapheme {
					matches: {
						let mut s = HashSet::new();
						s.insert("oo");
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
		assert_eq!(next, Choonpu("oo", false).into());
	}
}
