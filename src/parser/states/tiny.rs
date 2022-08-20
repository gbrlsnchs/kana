use crate::{
	config::KanaTable,
	parser::{
		machine::{Next, NextState, Previous},
		util,
	},
};

use super::{Choonpu, KanaToggle, Sukuon, Syllabogram, SHORT_SIZE};

pub const SIZE: usize = 1;

impl<'a> Next<'a> for Syllabogram<'a, SIZE> {
	const SIZE: usize = SIZE;

	fn next(self, table: &KanaTable<'a>) -> (Option<&'a str>, NextState<'a>) {
		let word = self.0;

		if util::utf8_word_count(word) < 1 {
			return (None, None);
		}

		let query = util::utf8_word_slice_until(word, Self::SIZE);

		(
			table
				.syllabograms
				.get(&query)
				.map_or_else(|| Some(query), |s| Some(*s)),
			if table.graphemes.choonpu.is_some() {
				Choonpu::prev(self).into()
			} else {
				KanaToggle::prev(self).into()
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
			util::utf8_word_slice_from(state.0, Choonpu::SIZE - 1),
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
	fn test_small_word<'a>() {
		let current = Syllabogram::<'a, SIZE>("", None);
		let table = KanaTable::default();
		let next = current.next(&table);

		assert_eq!((None, None), next);
	}

	#[test]
	fn test_no_match<'a>() {
		let current = Syllabogram::<'a, SIZE>("a", None);
		let table = KanaTable::default();
		let (result, next) = current.next(&table);

		// This state returns the original query if nothing is found, since it's essentially the
		// last parsing step, this way preserving untranslatable characters prev the original word.
		assert_eq!(result, Some("a"));
		assert_eq!(next, KanaToggle("", None, false).into());
	}

	#[test]
	fn test_regular_match<'a>() {
		let current = Syllabogram::<'a, SIZE>("abc", None);
		let table = KanaTable {
			syllabograms: HashMap::from([("a", "@")]),
			..Default::default()
		};
		let (result, next) = current.next(&table);

		assert_eq!(result, Some("@"));
		assert_eq!(next, KanaToggle("bc", None, false).into());
	}

	#[test]
	fn test_match_with_choonpu<'a>() {
		let current = Syllabogram::<'a, SIZE>("oomen", None);
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
	fn test_prev_monograph<'a>() {
		assert_eq!(
			Syllabogram::<'a, SIZE>::prev(Syllabogram::<'a, SHORT_SIZE>("testing", None)),
			Syllabogram::<'a, SIZE>("testing", None),
		);
		assert_eq!(
			Syllabogram::<'a, SIZE>::prev(Syllabogram::<'a, SHORT_SIZE>("testing", Some('@'))),
			Syllabogram::<'a, SIZE>("testing", Some('@')),
		);
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

	#[test]
	fn test_prev_choonpu<'a>() {
		assert_eq!(
			Syllabogram::<'a, SIZE>::prev(Choonpu("testing", None, false)),
			Syllabogram::<'a, SIZE>("esting", None),
		);
		assert_eq!(
			Syllabogram::<'a, SIZE>::prev(Choonpu("testing", Some('@'), false)),
			Syllabogram::<'a, SIZE>("esting", Some('@')),
		);
	}
}
