use crate::transliterate::Feature;

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
	Choonpu,
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
				Self::RawToggle => match input.special_chars.get(&Feature::RawText) {
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
				Self::KanaToggle => match input.special_chars.get(&Feature::Kana) {
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
							Self::Choonpu,
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
							Self::Choonpu,
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
						None => ("", input, Self::Short),
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
							Self::Choonpu,
						),
						None => ("", input, Self::Punctuation(size, &Self::Tiny)),
					}
				}
				Self::Tiny => {
					let selection = utf8::slice_to(romaji, size);

					match input.kanas.get_current().get(selection) {
						Some(output) => (output, input, Self::Choonpu),
						None => ("", input, Self::Punctuation(size, &Self::Fallback)),
					}
				}
				Self::Choonpu => match input.special_chars.get(&Feature::Reset) {
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

						match input.kanas.get_current().choonpu(selection) {
							Some(output) => (output, input, Self::Choonpu),
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
			| Self::Fallback => 1,
			Self::Short | Self::Sokuon | Self::Choonpu => 2,
			Self::Medium => 3,
			Self::Long => 4,
			Self::Punctuation(size, _) => *size,
		}
	}
}
