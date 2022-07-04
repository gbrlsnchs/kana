use crate::{
	config::KanaTable,
	parser::{
		machine::{Next, NextState, Previous},
		util,
	},
};

use super::{Choonpu, LongDigraph, Monograph, Sukuon};

#[derive(Debug, PartialEq)]
pub struct Nasal<'a>(pub &'a str);

impl<'a> Next<'a> for Nasal<'a> {
	const SIZE: usize = 1;

	fn next(self, table: &KanaTable<'a>) -> (Option<&'a str>, NextState<'a>) {
		let word = self.0;

		if util::utf8_word_count(word) < 1 {
			return (None, None);
		}

		let query = util::utf8_word_slice_until(word, Self::SIZE);

		(
			table
				.syllabograms
				.get(query)
				.map_or_else(|| Some(query), |s| Some(*s)),
			if table.graphemes.choonpu.is_some() {
				Choonpu::prev(self).into()
			} else {
				LongDigraph::prev(self).into()
			},
		)
	}
}

impl<'a> Previous<'a, Monograph<'a>> for Nasal<'a> {
	fn prev(state: Monograph<'a>) -> Self {
		Self(state.0)
	}
}

impl<'a> Previous<'a, Sukuon<'a>> for Nasal<'a> {
	fn prev(state: Sukuon<'a>) -> Self {
		Self(state.0)
	}
}

impl<'a> Previous<'a, Choonpu<'a>> for Nasal<'a> {
	fn prev(state: Choonpu<'a>) -> Self {
		Self(util::utf8_word_slice_from(state.0, Choonpu::SIZE - 1))
	}
}

#[cfg(test)]
mod tests {
	use std::collections::{HashMap, HashSet};

	use crate::config::{Grapheme, Graphemes};

	use super::*;

	#[test]
	fn test_small_word() {
		let current = Nasal("");
		let table = KanaTable::default();
		let next = current.next(&table);

		assert_eq!((None, None), next);
	}

	#[test]
	fn test_no_match() {
		let current = Nasal("a");
		let table = KanaTable::default();
		let (result, next) = current.next(&table);

		// This state returns the original query if nothing is found, since it's essentially the
		// last parsing step, this way preserving untranslatable characters prev the original word.
		assert_eq!(result, Some("a"));
		assert_eq!(next, LongDigraph("").into());
	}

	#[test]
	fn test_regular_match() {
		let current = Nasal("abc");
		let table = KanaTable {
			syllabograms: {
				let mut m = HashMap::new();
				m.insert("a", "@");
				m
			},
			..Default::default()
		};
		let (result, next) = current.next(&table);

		assert_eq!(result, Some("@"));
		assert_eq!(next, LongDigraph("bc").into());
	}

	#[test]
	fn test_match_with_choonpu() {
		let current = Nasal("oomen");
		let table = KanaTable {
			syllabograms: {
				let mut m = HashMap::new();
				m.insert("o", "@");
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
		assert_eq!(next, Choonpu("oomen", false).into());
	}

	#[test]
	fn test_prev_monograph() {
		assert_eq!(Nasal::prev(Monograph("testing")), Nasal("testing"));
	}

	#[test]
	fn test_prev_sukuon() {
		assert_eq!(Nasal::prev(Sukuon("testing")), Nasal("testing"));
	}

	#[test]
	fn test_prev_choonpu() {
		assert_eq!(Nasal::prev(Choonpu("testing", false)), Nasal("esting"));
	}
}
