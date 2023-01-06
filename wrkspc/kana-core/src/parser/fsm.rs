use std::hash::BuildHasher;

use crate::transliterate::Toggle;

use super::{
	glyphs::punctuation::{self, MARKS as PUNCTUATION_MARKS},
	input::{Input, Punctuation},
	utf8,
};

#[derive(Debug)]
pub enum State {
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
	Quote,
	Fallback,
}

impl<'a> State {
	pub fn next(self, mut input: Input<'a>) -> Option<(&'a str, Input<'a>, State)> {
		let size = self.get_size();
		let romaji = input.romaji;

		match self {
			Self::Init => match romaji.is_empty() {
				true => None,
				false => Some(("", input, Self::RawToggle)),
			},
			state => {
				Some(match state {
					Self::RawToggle => match input.toggles.get(&Toggle::RawText) {
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
					Self::KanaToggle => match input.toggles.get(&Toggle::Kana) {
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
							None => ("", input, Self::Medium),
						}
					}
					Self::Medium => {
						let selection = utf8::slice_to(romaji, size);
						let has_punctuation = input.punctuation.is_some();

						match input.kanas.get_current().get(selection).or_else(|| {
							match has_punctuation {
								true => PUNCTUATION_MARKS.get(selection).map(|s| *s),
								false => None,
							}
						}) {
							Some(output) => (
								output,
								{
									input.romaji = utf8::slice_from(romaji, size - 1);
									input
								},
								Self::Choonpu,
							),
							None => ("", input, Self::Sokuon),
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
							None => ("", input, Self::Tiny),
						}
					}
					Self::Tiny => {
						let selection = utf8::slice_to(romaji, size);
						let has_punctuation = input.punctuation.is_some();

						match input.kanas.get_current().get(selection).or_else(|| {
							match has_punctuation {
								true => PUNCTUATION_MARKS.get(selection).map(|s| *s),
								false => None,
							}
						}) {
							Some(output) => (output, input, Self::Choonpu),
							None => (
								"",
								input,
								match has_punctuation {
									true => Self::Quote,
									false => Self::Fallback,
								},
							),
						}
					}
					Self::Quote => {
						let selection = utf8::slice_to(romaji, size);
						let dst = Self::Init;

						match selection {
							"'" => {
								let output = input
									.punctuation
									.as_ref()
									.unwrap()
									.single_quotes
									.get_current();

								(
									output,
									{
										input.punctuation.as_mut().unwrap().single_quotes.toggle();
										input.romaji = utf8::slice_from(romaji, size);
										input
									},
									dst,
								)
							}
							r#"""# => {
								let output = input
									.punctuation
									.as_ref()
									.unwrap()
									.double_quotes
									.get_current();

								(
									output,
									{
										input.punctuation.as_mut().unwrap().double_quotes.toggle();
										input.romaji = utf8::slice_from(romaji, size);
										input
									},
									dst,
								)
							}
							_ => ("", input, Self::Fallback),
						}
					}
					Self::Choonpu => {
						let selection = utf8::slice_to(romaji, size);
						input.romaji = utf8::slice_from(romaji, size - 1);

						match input.kanas.get_current().choonpu(selection) {
							Some(output) => (output, input, Self::Choonpu),
							None => ("", input, Self::Init),
						}
					}
					Self::Fallback => {
						let output = utf8::slice_to(romaji, size);
						input.romaji = utf8::slice_from(romaji, size);

						(output, input, Self::Init)
					}
					Self::Init => unreachable!("this has already been checked above"),
				})
			}
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
			| Self::Quote
			| Self::Fallback => 1,
			Self::Short | Self::Sokuon | Self::Choonpu => 2,
			Self::Medium => 3,
			Self::Long => 4,
		}
	}
}
