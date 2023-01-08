use crate::transliterate::CharFeature;

use super::{glyphs::punctuation::MARKS as PUNCTUATION_MARKS, input::Input, utf8};

#[derive(Clone, Copy, Debug)]
pub enum State<'a> {
	Init,
	RawToggle,
	KanaToggle,
	SizeRouter,
	Long,
	Medium,
	Short,
	Tiny,
	Sokuon,
	VirtualSokuon,
	SmallVowel,
	Chouonpu,
	RawText(char),
	Punctuation(usize, &'a Self),
	Fallback,
}

impl<'a> State<'a> {
	pub fn next(self, mut input: Input<'a>) -> Option<(&'a str, Input<'a>, State)> {
		let size = self.get_size();
		let romaji = input.romaji;

		match self {
			Self::Init => match romaji.is_empty() {
				true => None,
				false => Some(("", input, Self::RawToggle)),
			},
			state => Some(match state {
				Self::RawToggle => match input.special_chars.get(&CharFeature::RawTextToggle) {
					Some(toggle) if romaji.starts_with(*toggle) => (
						"",
						{
							input.romaji = utf8::slice_from(romaji, size);
							input
						},
						Self::RawText(*toggle),
					),
					_ => ("", input, Self::KanaToggle),
				},
				Self::RawText(toggle) => {
					input.romaji = utf8::slice_from(romaji, size);

					match romaji.starts_with(toggle) {
						true => ("", input, Self::RawToggle),
						false => (
							utf8::slice_to(romaji, size),
							input,
							if romaji.is_empty() {
								Self::Init
							} else {
								Self::RawText(toggle)
							},
						),
					}
				}
				Self::KanaToggle => match input.special_chars.get(&CharFeature::KanaToggle) {
					Some(toggle) if input.romaji.starts_with(*toggle) => (
						"",
						{
							input.kanas.toggle();
							input.romaji = utf8::slice_from(romaji, size);
							input
						},
						Self::Init,
					),
					_ => ("", input, Self::SizeRouter),
				},
				Self::SizeRouter => (
					"",
					input,
					match utf8::count_chars(romaji) {
						n if n >= 4 => Self::Long,
						3 => Self::Medium,
						2 => Self::Sokuon,
						1 => Self::Tiny,
						_ => Self::Init,
					},
				),
				Self::Long => {
					let selection = utf8::slice_to(romaji, size);

					match input.kanas.get_current().get(selection) {
						Some(output) => (
							output,
							{
								input.romaji = utf8::slice_from(romaji, size - 1);
								input
							},
							Self::Chouonpu,
						),
						None => ("", input, Self::Punctuation(size, &Self::Medium)),
					}
				}
				Self::Medium => {
					let selection = utf8::slice_to(romaji, size);

					match input.kanas.get_current().get(selection) {
						Some(output) => (
							output,
							{
								input.romaji = utf8::slice_from(romaji, size - 1);
								input
							},
							Self::Chouonpu,
						),
						None => ("", input, Self::Punctuation(size, &Self::Sokuon)),
					}
				}
				Self::Sokuon => {
					let selection = utf8::slice_to(romaji, size);

					match input.kanas.get_current().sokuon(selection) {
						Some(output) => (
							output,
							{
								input.romaji = utf8::slice_from(romaji, size - 1);
								input
							},
							Self::Init,
						),
						None => ("", input, Self::SmallVowel),
					}
				}
				Self::SmallVowel => {
					let selection = utf8::slice_to(romaji, size);

					match input.special_chars.get(&CharFeature::SmallVowel) {
						Some(trigger) if selection.starts_with(*trigger) => {
							match input
								.kanas
								.get_current()
								.small_vowel(&utf8::slice_from(selection, 1))
							{
								Some(vowel) => (
									vowel,
									{
										input.romaji = utf8::slice_from(romaji, size - 1);
										input
									},
									Self::Chouonpu,
								),
								_ => ("", input, Self::Short),
							}
						}
						_ => ("", input, Self::Short),
					}
				}
				Self::Short => {
					let selection = utf8::slice_to(romaji, size);

					match input.kanas.get_current().get(selection) {
						Some(output) => (
							output,
							{
								input.romaji = utf8::slice_from(romaji, size - 1);
								input
							},
							Self::Chouonpu,
						),
						None => ("", input, Self::Punctuation(size, &Self::Tiny)),
					}
				}
				Self::Tiny => {
					let selection = utf8::slice_to(romaji, size);

					match input.kanas.get_current().get(selection) {
						Some(output) => (output, input, Self::Chouonpu),
						None => ("", input, Self::Punctuation(size, &Self::VirtualSokuon)),
					}
				}
				Self::Chouonpu => match input.special_chars.get(&CharFeature::Reset) {
					Some(reset) if utf8::slice_from(romaji, 1).starts_with(*reset) => (
						"",
						{
							input.romaji = utf8::slice_from(romaji, size);
							input
						},
						Self::Init,
					),
					_ => {
						let selection = utf8::slice_to(romaji, size);
						input.romaji = utf8::slice_from(romaji, size - 1);

						match input.kanas.get_current().chouonpu(selection) {
							Some(output) => (output, input, Self::Chouonpu),
							None => ("", input, Self::Init),
						}
					}
				},
				Self::Punctuation(size, next) => {
					let selection = utf8::slice_to(romaji, size);

					match input.punctuation {
						Some(ref mut punctuation) => match selection {
							"'" => {
								let output = punctuation.single_quotes.get_current();

								(
									output,
									{
										punctuation.single_quotes.toggle();
										input.romaji = utf8::slice_from(romaji, 1);
										input
									},
									Self::Init,
								)
							}
							r#"""# => {
								let output = punctuation.double_quotes.get_current();

								(
									output,
									{
										punctuation.double_quotes.toggle();
										input.romaji = utf8::slice_from(romaji, 1);
										input
									},
									Self::Init,
								)
							}
							_ => match PUNCTUATION_MARKS.get(selection) {
								Some(output) => (
									output,
									{
										input.romaji = utf8::slice_from(romaji, size);
										input
									},
									Self::Init,
								),
								None => ("", input, *next),
							},
						},
						None => ("", input, *next),
					}
				}
				Self::VirtualSokuon => match input.special_chars.get(&CharFeature::VirtualSokuon) {
					Some(c) if romaji.starts_with(*c) => (
						input.kanas.get_current().sokuon_literal(),
						{
							input.romaji = utf8::slice_from(romaji, size);
							input
						},
						Self::Init,
					),
					_ => ("", input, Self::Fallback),
				},
				Self::Fallback => {
					let output = utf8::slice_to(romaji, size);
					input.romaji = utf8::slice_from(romaji, size);

					(output, input, Self::Init)
				}
				Self::Init => unreachable!("this has already been checked above"),
			}),
		}
	}

	fn get_size(&self) -> usize {
		match self {
			Self::SizeRouter => 0,
			Self::Init
			| Self::RawToggle
			| Self::KanaToggle
			| Self::RawText(_)
			| Self::Tiny
			| Self::VirtualSokuon
			| Self::Fallback => 1,
			Self::Short | Self::Sokuon | Self::SmallVowel | Self::Chouonpu => 2,
			Self::Medium => 3,
			Self::Long => 4,
			Self::Punctuation(size, _) => *size,
		}
	}
}
