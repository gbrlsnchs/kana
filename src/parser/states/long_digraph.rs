use crate::{
	config::KanaTable,
	parser::{
		machine::{Next, NextState, Previous},
		util,
	},
};

use super::{Choonpu, KanaToggle, Sukuon};

#[derive(Debug, PartialEq)]
pub struct LongDigraph<'a>(pub &'a str, pub Option<char>);

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
					KanaToggle::prev(self).into()
				},
			),
		}
	}
}

impl<'a> Previous<'a, KanaToggle<'a>> for LongDigraph<'a> {
	fn prev(state: KanaToggle<'a>) -> Self {
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
		let current = LongDigraph("テスト", None);
		let table = KanaTable::default();
		let (result, next) = current.next(&table);

		assert_eq!(result, None);
		assert_eq!(next, Sukuon("テスト", None).into());
	}

	#[test]
	fn test_no_match() {
		let current = LongDigraph("testing", None);
		let table = KanaTable::default();
		let (result, next) = current.next(&table);

		assert_eq!(result, None);
		assert_eq!(next, Sukuon("testing", None).into());
	}

	#[test]
	fn test_regular_match() {
		let current = LongDigraph("testing", None);
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
		assert_eq!(next, KanaToggle("ing", None, false).into());
	}

	#[test]
	fn test_match_with_choonpu() {
		let current = LongDigraph("yahoo", None);
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
		assert_eq!(next, Choonpu("oo", None, false).into());
	}

	#[test]
	fn test_prev_kana_toggle() {
		assert_eq!(
			LongDigraph::prev(KanaToggle("testing", None, true)),
			LongDigraph("testing", None),
		);
		assert_eq!(
			LongDigraph::prev(KanaToggle("testing", Some('@'), true)),
			LongDigraph("testing", Some('@')),
		);
	}
}
