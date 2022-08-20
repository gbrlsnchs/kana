use crate::{
	config::KanaTable,
	parser::{
		machine::{Next, NextState, Previous},
		util,
	},
};

use super::{
	toggle::{Type as ToggleType, SIZE},
	Choonpu, Sukuon, Syllabogram, Toggle, CHOONPU_SIZE, LONG_SIZE, MEDIUM_SIZE, SHORT_SIZE,
	SUKUON_SIZE, TINY_SIZE,
};

pub const TYPE: usize = ToggleType::Kana as usize;

impl<'a> Next<'a> for Toggle<'a, TYPE> {
	fn next(self, _: &KanaTable<'a>) -> (Option<&'a str>, NextState<'a>) {
		let word = self.0;

		if self.1.map_or(false, |c| word.starts_with(c)) {
			(None, Self::prev(self).into())
		} else {
			(None, Syllabogram::<'a, LONG_SIZE>::prev(self).into())
		}
	}
}

impl<'a> Previous<'a, Self> for Toggle<'a, SIZE> {
	fn prev(state: Toggle<'a, SIZE>) -> Self {
		Self(util::utf8_word_slice_from(state.0, SIZE), state.1, true)
	}
}

impl<'a> Previous<'a, Syllabogram<'a, LONG_SIZE>> for Toggle<'a, SIZE> {
	fn prev(state: Syllabogram<'a, LONG_SIZE>) -> Self {
		Self(
			util::utf8_word_slice_from(state.0, LONG_SIZE),
			state.1,
			false,
		)
	}
}

impl<'a> Previous<'a, Syllabogram<'a, MEDIUM_SIZE>> for Toggle<'a, SIZE> {
	fn prev(state: Syllabogram<'a, MEDIUM_SIZE>) -> Self {
		Self(
			util::utf8_word_slice_from(state.0, MEDIUM_SIZE),
			state.1,
			false,
		)
	}
}

impl<'a> Previous<'a, Syllabogram<'a, SHORT_SIZE>> for Toggle<'a, SIZE> {
	fn prev(state: Syllabogram<'a, SHORT_SIZE>) -> Self {
		Self(
			util::utf8_word_slice_from(state.0, SHORT_SIZE),
			state.1,
			false,
		)
	}
}

impl<'a> Previous<'a, Syllabogram<'a, TINY_SIZE>> for Toggle<'a, SIZE> {
	fn prev(state: Syllabogram<'a, TINY_SIZE>) -> Self {
		Self(
			util::utf8_word_slice_from(state.0, TINY_SIZE),
			state.1,
			false,
		)
	}
}

impl<'a> Previous<'a, Sukuon<'a>> for Toggle<'a, SIZE> {
	fn prev(state: Sukuon<'a>) -> Self {
		Self(
			util::utf8_word_slice_from(state.0, SUKUON_SIZE - 1),
			state.1,
			false,
		)
	}
}

impl<'a> Previous<'a, Choonpu<'a>> for Toggle<'a, SIZE> {
	fn prev(state: Choonpu<'a>) -> Self {
		Self(
			util::utf8_word_slice_from(
				state.0,
				if state.2 {
					CHOONPU_SIZE
				} else {
					CHOONPU_SIZE - 1
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
	fn test_no_match_none() {
		let current = Toggle::<TYPE>("testing", None, false);
		let table = KanaTable::default();
		let (result, next) = current.next(&table);

		assert_eq!(result, None);
		assert_eq!(next, Syllabogram::<LONG_SIZE>("testing", None).into());
	}
	#[test]
	fn test_no_match_mismatch() {
		let current = Toggle::<TYPE>("@testing", Some('+'), false);
		let table = KanaTable::default();
		let (result, next) = current.next(&table);

		assert_eq!(result, None);
		assert_eq!(next, Syllabogram::<LONG_SIZE>("@testing", Some('+')).into());
	}

	#[test]
	fn test_match() {
		let current = Toggle::<TYPE>("@testing", Some('@'), false);
		let table = KanaTable::default();
		let (result, next) = current.next(&table);

		assert_eq!(result, None);
		assert_eq!(next, Toggle::<TYPE>("testing", Some('@'), true).into());
	}

	#[test]
	fn test_prev_kana_toggle() {
		assert_eq!(
			Toggle::<TYPE>::prev(Toggle::<TYPE>("@testing", None, false)),
			Toggle::<TYPE>("testing", None, true),
		);
		assert_eq!(
			Toggle::<TYPE>::prev(Toggle::<TYPE>("@testing", Some('@'), false)),
			Toggle::<TYPE>("testing", Some('@'), true),
		);
		assert_eq!(
			Toggle::<TYPE>::prev(Toggle::<TYPE>("@testing", None, true)),
			Toggle::<TYPE>("testing", None, true),
		);
		assert_eq!(
			Toggle::<TYPE>::prev(Toggle::<TYPE>("@testing", Some('@'), true)),
			Toggle::<TYPE>("testing", Some('@'), true),
		);
	}

	#[test]
	fn test_prev_long_digraph() {
		assert_eq!(
			Toggle::<TYPE>::prev(Syllabogram::<LONG_SIZE>("testing", None)),
			Toggle::<TYPE>("ing", None, false),
		);
		assert_eq!(
			Toggle::<TYPE>::prev(Syllabogram::<LONG_SIZE>("testing", Some('@'))),
			Toggle::<TYPE>("ing", Some('@'), false),
		);
	}

	#[test]
	fn test_prev_digraph() {
		assert_eq!(
			Toggle::<TYPE>::prev(Syllabogram::<MEDIUM_SIZE>("testing", None)),
			Toggle::<TYPE>("ting", None, false),
		);
		assert_eq!(
			Toggle::<TYPE>::prev(Syllabogram::<MEDIUM_SIZE>("testing", Some('@'))),
			Toggle::<TYPE>("ting", Some('@'), false),
		);
	}

	#[test]
	fn test_prev_monograph() {
		assert_eq!(
			Toggle::<TYPE>::prev(Syllabogram::<SHORT_SIZE>("testing", None)),
			Toggle::<TYPE>("sting", None, false),
		);
		assert_eq!(
			Toggle::<TYPE>::prev(Syllabogram::<SHORT_SIZE>("testing", Some('@'))),
			Toggle::<TYPE>("sting", Some('@'), false),
		);
	}

	#[test]
	fn test_prev_nasal() {
		assert_eq!(
			Toggle::<TYPE>::prev(Syllabogram::<TINY_SIZE>("testing", None)),
			Toggle::<TYPE>("esting", None, false),
		);
		assert_eq!(
			Toggle::<TYPE>::prev(Syllabogram::<TINY_SIZE>("testing", Some('@'))),
			Toggle::<TYPE>("esting", Some('@'), false),
		);
	}

	#[test]
	fn test_prev_sukuon() {
		assert_eq!(
			Toggle::<TYPE>::prev(Sukuon("testing", None)),
			Toggle::<TYPE>("esting", None, false),
		);
		assert_eq!(
			Toggle::<TYPE>::prev(Sukuon("testing", Some('@'))),
			Toggle::<TYPE>("esting", Some('@'), false),
		);
	}

	#[test]
	fn test_prev_choonpu() {
		assert_eq!(
			Toggle::<TYPE>::prev(Choonpu("testing", None, false)),
			Toggle::<TYPE>("esting", None, false),
		);
		assert_eq!(
			Toggle::<TYPE>::prev(Choonpu("testing", Some('@'), false)),
			Toggle::<TYPE>("esting", Some('@'), false),
		);
		assert_eq!(
			Toggle::<TYPE>::prev(Choonpu("testing", None, true)),
			Toggle::<TYPE>("sting", None, false),
		);
		assert_eq!(
			Toggle::<TYPE>::prev(Choonpu("testing", Some('@'), true)),
			Toggle::<TYPE>("sting", Some('@'), false),
		);
	}
}
