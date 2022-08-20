use crate::{
	config::KanaTable,
	parser::{
		machine::{Next, NextState, Previous},
		util,
	},
};

use super::{Choonpu, Sukuon, Syllabogram, Toggle, KANA_TOGGLE};

pub const SIZE: usize = 4;

impl<'a> Next<'a> for Syllabogram<'a, SIZE> {
	fn next(self, table: &KanaTable<'a>) -> (Option<&'a str>, NextState<'a>) {
		let word = self.0;

		if util::utf8_word_count(word) < SIZE {
			return (None, Sukuon::prev(self).into());
		}

		let query = util::utf8_word_slice_until(word, SIZE);

		match table.syllabograms.get(query) {
			None => (None, Sukuon::prev(self).into()),
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

impl<'a> Previous<'a, Toggle<'a, KANA_TOGGLE>> for Syllabogram<'a, SIZE> {
	fn prev(state: Toggle<'a, KANA_TOGGLE>) -> Self {
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
		let current = Syllabogram::<SIZE>("テスト", None);
		let table = KanaTable::default();
		let (result, next) = current.next(&table);

		assert_eq!(result, None);
		assert_eq!(next, Sukuon("テスト", None).into());
	}

	#[test]
	fn test_no_match() {
		let current = Syllabogram::<SIZE>("testing", None);
		let table = KanaTable::default();
		let (result, next) = current.next(&table);

		assert_eq!(result, None);
		assert_eq!(next, Sukuon("testing", None).into());
	}

	#[test]
	fn test_regular_match() {
		let current = Syllabogram::<SIZE>("testing", None);
		let table = KanaTable {
			syllabograms: HashMap::from([("test", "@")]),
			..Default::default()
		};
		let (result, next) = current.next(&table);

		assert_eq!(result, Some("@"));
		assert_eq!(next, Toggle::<KANA_TOGGLE>("ing", None, false).into());
	}

	#[test]
	fn test_match_with_choonpu() {
		let current = Syllabogram::<SIZE>("yahoo", None);
		let table = KanaTable {
			syllabograms: HashMap::from([("yaho", "@")]),
			graphemes: Graphemes {
				choonpu: Some(Grapheme {
					matches: HashSet::from(["oo"]),
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
			Syllabogram::<SIZE>::prev(Toggle::<KANA_TOGGLE>("testing", None, true)),
			Syllabogram::<SIZE>("testing", None),
		);
		assert_eq!(
			Syllabogram::<SIZE>::prev(Toggle::<KANA_TOGGLE>("testing", Some('@'), true)),
			Syllabogram::<SIZE>("testing", Some('@')),
		);
	}
}
