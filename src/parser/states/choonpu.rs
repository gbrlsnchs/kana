use crate::{
	config::KanaTable,
	parser::{
		machine::{self, Next, Previous},
		util,
	},
};

use super::{Digraph, KanaToggle, LongDigraph, Monograph, Nasal};

#[derive(Debug, PartialEq)]
pub struct Choonpu<'a>(pub &'a str, pub Option<char>, pub bool);

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
				self.2 = true;

				return (Some(choonpu.graph), KanaToggle::prev(self).into());
			}
		}

		(None, KanaToggle::prev(self).into())
	}
}

impl<'a> Previous<'a, LongDigraph<'a>> for Choonpu<'a> {
	fn prev(state: LongDigraph<'a>) -> Self {
		Self(
			util::utf8_word_slice_from(state.0, LongDigraph::SIZE - 1),
			state.1,
			false,
		)
	}
}

impl<'a> Previous<'a, Digraph<'a>> for Choonpu<'a> {
	fn prev(state: Digraph<'a>) -> Self {
		Self(
			util::utf8_word_slice_from(state.0, Digraph::SIZE - 1),
			state.1,
			false,
		)
	}
}

impl<'a> Previous<'a, Monograph<'a>> for Choonpu<'a> {
	fn prev(state: Monograph<'a>) -> Self {
		Self(
			util::utf8_word_slice_from(state.0, Monograph::SIZE - 1),
			state.1,
			false,
		)
	}
}

impl<'a> Previous<'a, Nasal<'a>> for Choonpu<'a> {
	fn prev(state: Nasal<'a>) -> Self {
		Self(state.0, state.1, false)
	}
}

#[cfg(test)]
mod tests {
	use std::collections::{HashMap, HashSet};

	use crate::config::{Grapheme, Graphemes};

	use super::*;

	#[test]
	fn test_small_word() {
		let current = Choonpu("ツ", None, false);
		let table = KanaTable::default();
		let next = current.next(&table);

		assert_eq!((None, Nasal("", None).into()), next);
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
		assert_eq!(next, KanaToggle("men", None, false).into());
	}

	#[test]
	fn test_prev_long_digraph() {
		assert_eq!(
			Choonpu::prev(LongDigraph("testing", None)),
			Choonpu("ting", None, false),
		);
		assert_eq!(
			Choonpu::prev(LongDigraph("testing", Some('@'))),
			Choonpu("ting", Some('@'), false),
		);
	}

	#[test]
	fn test_prev_digraph() {
		assert_eq!(
			Choonpu::prev(Digraph("testing", None)),
			Choonpu("sting", None, false)
		);
		assert_eq!(
			Choonpu::prev(Digraph("testing", Some('@'))),
			Choonpu("sting", Some('@'), false)
		);
	}

	#[test]
	fn test_prev_monograph() {
		assert_eq!(
			Choonpu::prev(Monograph("testing", None)),
			Choonpu("esting", None, false)
		);
		assert_eq!(
			Choonpu::prev(Monograph("testing", Some('@'))),
			Choonpu("esting", Some('@'), false)
		);
	}

	#[test]
	fn test_prev_nasal() {
		assert_eq!(
			Choonpu::prev(Nasal("testing", None)),
			Choonpu("testing", None, false)
		);
		assert_eq!(
			Choonpu::prev(Nasal("testing", Some('@'))),
			Choonpu("testing", Some('@'), false)
		);
	}
}
