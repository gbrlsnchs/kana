use crate::{
	config::KanaTable,
	parser::{
		machine::{Next, NextState, Previous},
		util,
	},
};

use super::{Choonpu, KanaToggle, Syllabogram, MEDIUM_SIZE, TINY_SIZE};

pub const SIZE: usize = 2;

impl<'a> Next<'a> for Syllabogram<'a, SIZE> {
	const SIZE: usize = SIZE;

	fn next(self, table: &KanaTable<'a>) -> (Option<&'a str>, NextState<'a>) {
		let word = self.0;

		if util::utf8_word_count(word) < Self::SIZE {
			return (None, Syllabogram::<'a, TINY_SIZE>::prev(self).into());
		}

		let query = util::utf8_word_slice_until(word, Self::SIZE);

		match table.syllabograms.get(query) {
			None => (None, Syllabogram::<'a, TINY_SIZE>::prev(self).into()),
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

impl<'a> Previous<'a, Syllabogram<'a, MEDIUM_SIZE>> for Syllabogram<'a, SIZE> {
	fn prev(state: Syllabogram<'a, MEDIUM_SIZE>) -> Self {
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
		let current = Syllabogram::<'a, SIZE>("あ", None);
		let table = KanaTable::default();
		let (result, next) = current.next(&table);

		assert_eq!(result, None);
		assert_eq!(next, Syllabogram::<'a, TINY_SIZE>("あ", None).into());
	}

	#[test]
	fn test_no_match<'a>() {
		let current = Syllabogram::<'a, SIZE>("alo", None);
		let table = KanaTable::default();
		let (result, next) = current.next(&table);

		assert_eq!(result, None);
		assert_eq!(next, Syllabogram::<'a, TINY_SIZE>("alo", None).into());
	}

	#[test]
	fn test_regular_match<'a>() {
		let current = Syllabogram::<'a, SIZE>("test", None);
		let table = KanaTable {
			syllabograms: HashMap::from([("te", "@")]),
			..Default::default()
		};
		let (result, next) = current.next(&table);

		assert_eq!(result, Some("@"));
		assert_eq!(next, KanaToggle("st", None, false).into());
	}

	#[test]
	fn test_match_with_choonpu<'a>() {
		let current = Syllabogram::<'a, SIZE>("oii", None);
		let table = KanaTable {
			syllabograms: HashMap::from([("oi", "@")]),
			graphemes: Graphemes {
				choonpu: Some(Grapheme {
					matches: HashSet::from(["ii"]),
					graph: "!",
				}),
				..Default::default()
			},
			..Default::default()
		};
		let (result, next) = current.next(&table);

		assert_eq!(result, Some("@"));
		assert_eq!(next, Choonpu("ii", None, false).into());
	}

	#[test]
	fn test_prev_digraph<'a>() {
		assert_eq!(
			Syllabogram::<'a, SIZE>::prev(Syllabogram::<'a, MEDIUM_SIZE>("testing", None)),
			Syllabogram::<'a, SIZE>("testing", None),
		);
		assert_eq!(
			Syllabogram::<'a, SIZE>::prev(Syllabogram::<'a, MEDIUM_SIZE>("testing", Some('@'))),
			Syllabogram::<'a, SIZE>("testing", Some('@')),
		);
	}
}
