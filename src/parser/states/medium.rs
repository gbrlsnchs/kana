use crate::{
	config::KanaTable,
	parser::{
		machine::{Next, NextState, Previous},
		util,
	},
};

use super::{sukuon::Sukuon, Choonpu, KanaToggle, Syllabogram, SHORT_SIZE};

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
					KanaToggle::prev(self).into()
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
	fn test_small_word<'a>() {
		let current = Syllabogram::<'a, SIZE>("oi", None);
		let table = KanaTable::default();
		let (result, next) = current.next(&table);

		assert_eq!(result, None);
		assert_eq!(next, Syllabogram::<'a, SHORT_SIZE>("oi", None).into());
	}

	#[test]
	fn test_no_match<'a>() {
		let current = Syllabogram::<'a, SIZE>("alo", None);
		let table = KanaTable::default();
		let (result, next) = current.next(&table);

		assert_eq!(result, None);
		assert_eq!(next, Syllabogram::<'a, SHORT_SIZE>("alo", None).into());
	}

	#[test]
	fn test_regular_match<'a>() {
		let current = Syllabogram::<'a, SIZE>("foobar", None);
		let table = KanaTable {
			syllabograms: HashMap::from([("foo", "@")]),
			..Default::default()
		};
		let (result, next) = current.next(&table);

		assert_eq!(result, Some("@"));
		assert_eq!(next, KanaToggle("bar", None, false).into());
	}

	#[test]
	fn test_match_with_choonpu<'a>() {
		let current = Syllabogram::<'a, SIZE>("olaa", None);
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
	fn test_prev_sukuon<'a>() {
		assert_eq!(
			Syllabogram::<'a, SIZE>::prev(Sukuon("testing", None)),
			Syllabogram::<'a, SIZE>("testing", None)
		);
		assert_eq!(
			Syllabogram::<'a, SIZE>::prev(Sukuon("testing", Some('@'))),
			Syllabogram::<'a, SIZE>("testing", Some('@')),
		);
	}
}
