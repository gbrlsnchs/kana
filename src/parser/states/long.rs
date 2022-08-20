use crate::{
	config::KanaTable,
	parser::{
		machine::{Next, NextState, Previous},
		util,
	},
};

use super::{Choonpu, KanaToggle, Sukuon, Syllabogram};

pub const SIZE: usize = 4;

impl<'a> Next<'a> for Syllabogram<'a, SIZE> {
	const SIZE: usize = SIZE;

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

impl<'a> Previous<'a, KanaToggle<'a>> for Syllabogram<'a, SIZE> {
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
	fn test_small_word<'a>() {
		let current = Syllabogram::<'a, SIZE>("テスト", None);
		let table = KanaTable::default();
		let (result, next) = current.next(&table);

		assert_eq!(result, None);
		assert_eq!(next, Sukuon("テスト", None).into());
	}

	#[test]
	fn test_no_match<'a>() {
		let current = Syllabogram::<'a, SIZE>("testing", None);
		let table = KanaTable::default();
		let (result, next) = current.next(&table);

		assert_eq!(result, None);
		assert_eq!(next, Sukuon("testing", None).into());
	}

	#[test]
	fn test_regular_match<'a>() {
		let current = Syllabogram::<'a, SIZE>("testing", None);
		let table = KanaTable {
			syllabograms: HashMap::from([("test", "@")]),
			..Default::default()
		};
		let (result, next) = current.next(&table);

		assert_eq!(result, Some("@"));
		assert_eq!(next, KanaToggle("ing", None, false).into());
	}

	#[test]
	fn test_match_with_choonpu<'a>() {
		let current = Syllabogram::<'a, SIZE>("yahoo", None);
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
	fn test_prev_kana_toggle<'a>() {
		assert_eq!(
			Syllabogram::<'a, SIZE>::prev(KanaToggle("testing", None, true)),
			Syllabogram::<'a, SIZE>("testing", None),
		);
		assert_eq!(
			Syllabogram::<'a, SIZE>::prev(KanaToggle("testing", Some('@'), true)),
			Syllabogram::<'a, SIZE>("testing", Some('@')),
		);
	}
}
