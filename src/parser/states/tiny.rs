use crate::{
	config::KanaTable,
	parser::{
		machine::{Next, NextState, Previous},
		util,
	},
};

use super::{Choonpu, Sukuon, Syllabogram, Toggle, CHOONPU_SIZE, KANA_TOGGLE, SHORT_SIZE};

pub const SIZE: usize = 1;

impl<'a> Next<'a> for Syllabogram<'a, SIZE> {
	fn next(self, table: &KanaTable<'a>) -> (Option<&'a str>, NextState<'a>) {
		let word = self.0;

		if util::utf8_word_count(word) < 1 {
			return (None, None);
		}

		let query = util::utf8_word_slice_until(word, SIZE);

		(
			table
				.syllabograms
				.get(&query)
				.map_or_else(|| Some(query), |s| Some(*s)),
			if table.graphemes.choonpu.is_some() {
				Choonpu::prev(self).into()
			} else {
				Toggle::<KANA_TOGGLE>::prev(self).into()
			},
		)
	}
}

impl<'a> Previous<'a, Syllabogram<'a, SHORT_SIZE>> for Syllabogram<'a, SIZE> {
	fn prev(state: Syllabogram<'a, SHORT_SIZE>) -> Self {
		Self(state.0, state.1)
	}
}

impl<'a> Previous<'a, Sukuon<'a>> for Syllabogram<'a, SIZE> {
	fn prev(state: Sukuon<'a>) -> Self {
		Self(state.0, state.1)
	}
}

impl<'a> Previous<'a, Choonpu<'a>> for Syllabogram<'a, SIZE> {
	fn prev(state: Choonpu<'a>) -> Self {
		Self(
			util::utf8_word_slice_from(state.0, CHOONPU_SIZE - 1),
			state.1,
		)
	}
}

#[cfg(test)]
mod tests {
	use std::collections::{HashMap, HashSet};

	use crate::config::{Grapheme, Graphemes};

	use super::*;

	#[test]
	fn test_small_word() {
		let current = Syllabogram::<SIZE>("", None);
		let table = KanaTable::default();
		let next = current.next(&table);

		assert_eq!((None, None), next);
	}

	#[test]
	fn test_no_match() {
		let current = Syllabogram::<SIZE>("a", None);
		let table = KanaTable::default();
		let (result, next) = current.next(&table);

		// This state returns the original query if nothing is found, since it's essentially the
		// last parsing step, this way preserving untranslatable characters prev the original word.
		assert_eq!(result, Some("a"));
		assert_eq!(next, Toggle::<KANA_TOGGLE>("", None, false).into());
	}

	#[test]
	fn test_regular_match() {
		let current = Syllabogram::<SIZE>("abc", None);
		let table = KanaTable {
			syllabograms: HashMap::from([("a", "@")]),
			..Default::default()
		};
		let (result, next) = current.next(&table);

		assert_eq!(result, Some("@"));
		assert_eq!(next, Toggle::<KANA_TOGGLE>("bc", None, false).into());
	}

	#[test]
	fn test_match_with_choonpu() {
		let current = Syllabogram::<SIZE>("oomen", None);
		let table = KanaTable {
			syllabograms: HashMap::from([("o", "@")]),
			graphemes: Graphemes {
				choonpu: Some(Grapheme {
					matches: HashSet::from([("oo")]),
					graph: "!",
				}),
				..Default::default()
			},
			..Default::default()
		};
		let (result, next) = current.next(&table);

		assert_eq!(result, Some("@"));
		assert_eq!(next, Choonpu("oomen", None, false).into());
	}

	#[test]
	fn test_prev_monograph() {
		assert_eq!(
			Syllabogram::<SIZE>::prev(Syllabogram::<SHORT_SIZE>("testing", None)),
			Syllabogram::<SIZE>("testing", None),
		);
		assert_eq!(
			Syllabogram::<SIZE>::prev(Syllabogram::<SHORT_SIZE>("testing", Some('@'))),
			Syllabogram::<SIZE>("testing", Some('@')),
		);
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

	#[test]
	fn test_prev_choonpu() {
		assert_eq!(
			Syllabogram::<SIZE>::prev(Choonpu("testing", None, false)),
			Syllabogram::<SIZE>("esting", None),
		);
		assert_eq!(
			Syllabogram::<SIZE>::prev(Choonpu("testing", Some('@'), false)),
			Syllabogram::<SIZE>("esting", Some('@')),
		);
	}
}
