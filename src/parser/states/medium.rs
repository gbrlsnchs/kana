use crate::{
	config::KanaTable,
	parser::{
		machine::{Next, NextState, Previous},
		util,
	},
};

use super::{sukuon::Sukuon, Choonpu, Syllabogram, Toggle, KANA_TOGGLE, SHORT_SIZE};

pub const SIZE: usize = 3;

impl<'a> Next<'a> for Syllabogram<'a, SIZE> {
	fn next(self, table: &KanaTable<'a>) -> (Option<&'a str>, NextState<'a>) {
		let word = self.0;

		if util::utf8_word_count(word) < SIZE {
			return (None, Syllabogram::<'a, SHORT_SIZE>::prev(self).into());
		}

		let query = util::utf8_word_slice_until(word, SIZE);

		match table.syllabograms.get(query) {
			None => (None, Syllabogram::<'a, SHORT_SIZE>::prev(self).into()),
			Some(s) => (
				Some(s),
				if table.graphemes.choonpu.is_some() {
					Choonpu::prev(self).into()
				} else {
					Toggle::<KANA_TOGGLE>::prev(self).into()
				},
			),
		}
	}
}

impl<'a> Previous<'a, Sukuon<'a>> for Syllabogram<'a, SIZE> {
	fn prev(state: Sukuon<'a>) -> Self {
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
		let current = Syllabogram::<SIZE>("oi", None);
		let table = KanaTable::default();
		let (result, next) = current.next(&table);

		assert_eq!(result, None);
		assert_eq!(next, Syllabogram::<SHORT_SIZE>("oi", None).into());
	}

	#[test]
	fn test_no_match() {
		let current = Syllabogram::<SIZE>("alo", None);
		let table = KanaTable::default();
		let (result, next) = current.next(&table);

		assert_eq!(result, None);
		assert_eq!(next, Syllabogram::<SHORT_SIZE>("alo", None).into());
	}

	#[test]
	fn test_regular_match() {
		let current = Syllabogram::<SIZE>("foobar", None);
		let table = KanaTable {
			syllabograms: HashMap::from([("foo", "@")]),
			..Default::default()
		};
		let (result, next) = current.next(&table);

		assert_eq!(result, Some("@"));
		assert_eq!(next, Toggle::<KANA_TOGGLE>("bar", None, false).into());
	}

	#[test]
	fn test_match_with_choonpu() {
		let current = Syllabogram::<SIZE>("olaa", None);
		let table = KanaTable {
			syllabograms: HashMap::from([("ola", "@")]),
			graphemes: Graphemes {
				choonpu: Some(Grapheme {
					matches: HashSet::from(["aa"]),
					graph: "!",
				}),
				..Default::default()
			},
			..Default::default()
		};
		let (result, next) = current.next(&table);

		assert_eq!(result, Some("@"));
		assert_eq!(next, Choonpu("aa", None, false).into());
	}

	#[test]
	fn test_prev_sukuon() {
		assert_eq!(
			Syllabogram::<SIZE>::prev(Sukuon("testing", None)),
			Syllabogram::<SIZE>("testing", None)
		);
		assert_eq!(
			Syllabogram::<SIZE>::prev(Sukuon("testing", Some('@'))),
			Syllabogram::<SIZE>("testing", Some('@')),
		);
	}
}
