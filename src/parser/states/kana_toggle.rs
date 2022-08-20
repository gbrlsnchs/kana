use crate::{
	config::KanaTable,
	parser::{
		machine::{Next, NextState, Previous},
		util,
	},
};

use super::{Choonpu, Sukuon, Syllabogram, LONG_SIZE, MEDIUM_SIZE, SHORT_SIZE, TINY_SIZE};

#[derive(Debug, PartialEq)]
pub struct KanaToggle<'a>(pub &'a str, pub Option<char>, pub bool);

impl<'a> Next<'a> for KanaToggle<'a> {
	const SIZE: usize = 1;

	fn next(self, _: &KanaTable<'a>) -> (Option<&'a str>, NextState<'a>) {
		let word = self.0;

		if self.1.map_or(false, |c| word.starts_with(c)) {
			(None, Self::prev(self).into())
		} else {
			(None, Syllabogram::<'a, LONG_SIZE>::prev(self).into())
		}
	}
}

impl<'a> Previous<'a, Self> for KanaToggle<'a> {
	fn prev(state: KanaToggle<'a>) -> Self {
		Self(
			util::utf8_word_slice_from(state.0, Self::SIZE),
			state.1,
			true,
		)
	}
}

impl<'a> Previous<'a, Syllabogram<'a, LONG_SIZE>> for KanaToggle<'a> {
	fn prev(state: Syllabogram<'a, LONG_SIZE>) -> Self {
		Self(
			util::utf8_word_slice_from(state.0, Syllabogram::<'a, LONG_SIZE>::SIZE),
			state.1,
			false,
		)
	}
}

impl<'a> Previous<'a, Syllabogram<'a, MEDIUM_SIZE>> for KanaToggle<'a> {
	fn prev(state: Syllabogram<'a, MEDIUM_SIZE>) -> Self {
		Self(
			util::utf8_word_slice_from(state.0, Syllabogram::<'a, MEDIUM_SIZE>::SIZE),
			state.1,
			false,
		)
	}
}

impl<'a> Previous<'a, Syllabogram<'a, SHORT_SIZE>> for KanaToggle<'a> {
	fn prev(state: Syllabogram<'a, SHORT_SIZE>) -> Self {
		Self(
			util::utf8_word_slice_from(state.0, Syllabogram::<'a, SHORT_SIZE>::SIZE),
			state.1,
			false,
		)
	}
}

impl<'a> Previous<'a, Syllabogram<'a, TINY_SIZE>> for KanaToggle<'a> {
	fn prev(state: Syllabogram<'a, TINY_SIZE>) -> Self {
		Self(
			util::utf8_word_slice_from(state.0, Syllabogram::<'a, TINY_SIZE>::SIZE),
			state.1,
			false,
		)
	}
}

impl<'a> Previous<'a, Sukuon<'a>> for KanaToggle<'a> {
	fn prev(state: Sukuon<'a>) -> Self {
		Self(
			util::utf8_word_slice_from(state.0, Sukuon::SIZE - 1),
			state.1,
			false,
		)
	}
}

impl<'a> Previous<'a, Choonpu<'a>> for KanaToggle<'a> {
	fn prev(state: Choonpu<'a>) -> Self {
		Self(
			util::utf8_word_slice_from(
				state.0,
				if state.2 {
					Choonpu::SIZE
				} else {
					Choonpu::SIZE - 1
				},
			),
			state.1,
			false,
		)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_no_match_none<'a>() {
		let current = KanaToggle("testing", None, false);
		let table = KanaTable::default();
		let (result, next) = current.next(&table);

		assert_eq!(result, None);
		assert_eq!(next, Syllabogram::<'a, LONG_SIZE>("testing", None).into());
	}
	#[test]
	fn test_no_match_mismatch<'a>() {
		let current = KanaToggle("@testing", Some('+'), false);
		let table = KanaTable::default();
		let (result, next) = current.next(&table);

		assert_eq!(result, None);
		assert_eq!(
			next,
			Syllabogram::<'a, LONG_SIZE>("@testing", Some('+')).into()
		);
	}

	#[test]
	fn test_match() {
		let current = KanaToggle("@testing", Some('@'), false);
		let table = KanaTable::default();
		let (result, next) = current.next(&table);

		assert_eq!(result, None);
		assert_eq!(next, KanaToggle("testing", Some('@'), true).into());
	}

	#[test]
	fn test_prev_kana_toggle() {
		assert_eq!(
			KanaToggle::prev(KanaToggle("@testing", None, false)),
			KanaToggle("testing", None, true),
		);
		assert_eq!(
			KanaToggle::prev(KanaToggle("@testing", Some('@'), false)),
			KanaToggle("testing", Some('@'), true),
		);
		assert_eq!(
			KanaToggle::prev(KanaToggle("@testing", None, true)),
			KanaToggle("testing", None, true),
		);
		assert_eq!(
			KanaToggle::prev(KanaToggle("@testing", Some('@'), true)),
			KanaToggle("testing", Some('@'), true),
		);
	}

	#[test]
	fn test_prev_long_digraph<'a>() {
		assert_eq!(
			KanaToggle::prev(Syllabogram::<'a, LONG_SIZE>("testing", None)),
			KanaToggle("ing", None, false),
		);
		assert_eq!(
			KanaToggle::prev(Syllabogram::<'a, LONG_SIZE>("testing", Some('@'))),
			KanaToggle("ing", Some('@'), false),
		);
	}

	#[test]
	fn test_prev_digraph<'a>() {
		assert_eq!(
			KanaToggle::prev(Syllabogram::<'a, MEDIUM_SIZE>("testing", None)),
			KanaToggle("ting", None, false),
		);
		assert_eq!(
			KanaToggle::prev(Syllabogram::<'a, MEDIUM_SIZE>("testing", Some('@'))),
			KanaToggle("ting", Some('@'), false),
		);
	}

	#[test]
	fn test_prev_monograph<'a>() {
		assert_eq!(
			KanaToggle::prev(Syllabogram::<'a, SHORT_SIZE>("testing", None)),
			KanaToggle("sting", None, false),
		);
		assert_eq!(
			KanaToggle::prev(Syllabogram::<'a, SHORT_SIZE>("testing", Some('@'))),
			KanaToggle("sting", Some('@'), false),
		);
	}

	#[test]
	fn test_prev_nasal<'a>() {
		assert_eq!(
			KanaToggle::prev(Syllabogram::<'a, TINY_SIZE>("testing", None)),
			KanaToggle("esting", None, false),
		);
		assert_eq!(
			KanaToggle::prev(Syllabogram::<'a, TINY_SIZE>("testing", Some('@'))),
			KanaToggle("esting", Some('@'), false),
		);
	}

	#[test]
	fn test_prev_sukuon() {
		assert_eq!(
			KanaToggle::prev(Sukuon("testing", None)),
			KanaToggle("esting", None, false),
		);
		assert_eq!(
			KanaToggle::prev(Sukuon("testing", Some('@'))),
			KanaToggle("esting", Some('@'), false),
		);
	}

	#[test]
	fn test_prev_choonpu() {
		assert_eq!(
			KanaToggle::prev(Choonpu("testing", None, false)),
			KanaToggle("esting", None, false),
		);
		assert_eq!(
			KanaToggle::prev(Choonpu("testing", Some('@'), false)),
			KanaToggle("esting", Some('@'), false),
		);
		assert_eq!(
			KanaToggle::prev(Choonpu("testing", None, true)),
			KanaToggle("sting", None, false),
		);
		assert_eq!(
			KanaToggle::prev(Choonpu("testing", Some('@'), true)),
			KanaToggle("sting", Some('@'), false),
		);
	}
}
