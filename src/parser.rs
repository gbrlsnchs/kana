use crate::spec::Spec;

use super::state::State;

pub type Computation<'a> = (Option<&'a str>, State<'a>);

#[derive(Debug, PartialEq)]
pub enum Section {
	Digraph = 0,
	Monograph,
	Nasal,
	Sukuon,
	Choonpu,
	LongDigraph,
}

impl Default for Section {
	fn default() -> Self {
		Self::LongDigraph
	}
}

pub struct Parser<const S: usize>;

impl<const S: usize> Parser<S> {
	fn utf8_word_count(word: &str) -> usize {
		word.chars().count()
	}

	fn utf8_word_slice_size(word: &str, n: usize) -> usize {
		word.chars().take(n).map(|c| c.len_utf8()).sum()
	}
}

/// This is currently only used to handle 'tsyu' from Katakana.
impl Parser<{ Section::LongDigraph as usize }> {
	pub(super) fn next<'a>(spec: &'a Spec, word: &'a str) -> Option<Computation<'a>> {
		if Self::utf8_word_count(&word) < 4 {
			return Some((
				None,
				State {
					word,
					section: Section::Sukuon,
				},
			));
		}

		let size = Self::utf8_word_slice_size(&word, 4);

		match spec.graphs.get(&word[..size]) {
			Some(symbol) => {
				let has_choonpu = spec.graphemes.choonpu.is_some();
				let (size, section) = if has_choonpu {
					(3, Section::Choonpu)
				} else {
					(size, Section::default())
				};

				Some((
					Some(symbol),
					State {
						word: &word[size..],
						section,
					},
				))
			}
			None => Some((
				None,
				State {
					word,
					section: Section::Sukuon,
				},
			)),
		}
	}
}

impl Parser<{ Section::Digraph as usize }> {
	pub(super) fn next<'a>(spec: &'a Spec, word: &'a str) -> Option<Computation<'a>> {
		if Self::utf8_word_count(&word) < 3 {
			return Some((
				None,
				State {
					word,
					section: Section::Monograph,
				},
			));
		}

		let size = Self::utf8_word_slice_size(&word, 3);

		match spec.graphs.get(&word[..size]) {
			Some(symbol) => {
				let has_choonpu = spec.graphemes.choonpu.is_some();
				let (size, section) = if has_choonpu {
					(2, Section::Choonpu)
				} else {
					(size, Section::default())
				};

				Some((
					Some(symbol),
					State {
						word: &word[size..],
						section,
					},
				))
			}
			None => Some((
				None,
				State {
					word,
					section: Section::Monograph,
				},
			)),
		}
	}
}

impl Parser<{ Section::Monograph as usize }> {
	pub(super) fn next<'a>(spec: &'a Spec, word: &'a str) -> Option<Computation<'a>> {
		if Self::utf8_word_count(&word) < 2 {
			return Some((
				None,
				State {
					word,
					section: Section::Nasal,
				},
			));
		}

		let size = Self::utf8_word_slice_size(&word, 2);

		match spec.graphs.get(&word[..size]) {
			Some(symbol) => {
				let has_choonpu = spec.graphemes.choonpu.is_some();
				let (size, section) = if has_choonpu {
					(1, Section::Choonpu)
				} else {
					(size, Section::default())
				};

				Some((
					Some(symbol),
					State {
						word: &word[size..],
						section,
					},
				))
			}
			None => Some((
				None,
				State {
					word,
					section: Section::Nasal,
				},
			)),
		}
	}
}

impl Parser<{ Section::Sukuon as usize }> {
	pub(super) fn next<'a>(spec: &'a Spec, word: &'a str) -> Option<Computation<'a>> {
		if Self::utf8_word_count(&word) < 2 {
			return Some((
				None,
				State {
					word,
					section: Section::Nasal,
				},
			));
		}

		let size = Self::utf8_word_slice_size(&word, 2);

		if spec.graphemes.sukuon.matches.contains(&word[..size]) {
			let size = Self::utf8_word_slice_size(&word, 1);

			return Some((
				Some(&spec.graphemes.sukuon.graph),
				State {
					word: &word[size..],
					section: Section::default(),
				},
			));
		}

		Some((
			None,
			State {
				word,
				section: Section::Digraph,
			},
		))
	}
}

impl Parser<{ Section::Choonpu as usize }> {
	pub(super) fn next<'a>(spec: &'a Spec, word: &'a str) -> Option<Computation<'a>> {
		let shim_size = Self::utf8_word_slice_size(&word, 1);

		if Self::utf8_word_count(&word) < 2 {
			return Some((
				None,
				State {
					word: &word[shim_size..],
					section: Section::Nasal,
				},
			));
		}

		if let Some(choonpu) = &spec.graphemes.choonpu {
			let size = Self::utf8_word_slice_size(&word, 2);

			if choonpu.matches.contains(&word[..size]) {
				return Some((
					Some(choonpu.graph),
					State {
						word: &word[size..],
						section: Section::default(),
					},
				));
			}
		}

		Some((
			None,
			State {
				word: &word[shim_size..],
				section: Section::default(),
			},
		))
	}
}

impl Parser<{ Section::Nasal as usize }> {
	pub(super) fn next<'a>(spec: &'a Spec, word: &'a str) -> Option<Computation<'a>> {
		if Self::utf8_word_count(&word) < 1 {
			return None;
		}

		let size = Self::utf8_word_slice_size(&word, 1);
		let key = &word[..size];

		Some((
			spec.graphs.get(key).map(|s| *s).or_else(|| Some(key)),
			State {
				word: &word[size..],
				section: Section::default(),
			},
		))
	}
}

#[cfg(test)]
mod tests {
	use std::collections::HashSet;

	use super::*;

	#[test]
	fn test_long_diagraph() {
		let mut spec = Spec::default();

		{
			let (result, next_state) =
				Parser::<{ Section::LongDigraph as usize }>::next(&spec, "foobar").unwrap();

			assert_eq!(result, None);
			assert_eq!(
				next_state,
				State {
					word: "foobar",
					section: Section::Sukuon,
				}
			);
		}

		spec.graphs.insert("kata", "@");
		{
			let (result, next_state) =
				Parser::<{ Section::LongDigraph as usize }>::next(&spec, "katakana").unwrap();

			assert_eq!(result, Some("@"));
			assert_eq!(
				next_state,
				State {
					word: "kana",
					section: Section::default(),
				}
			);
		}

		{
			let (result, next_state) =
				Parser::<{ Section::LongDigraph as usize }>::next(&spec, "foo").unwrap();

			assert_eq!(result, None);
			assert_eq!(
				next_state,
				State {
					word: "foo",
					section: Section::Sukuon,
				}
			);
		}
	}

	#[test]
	fn test_diagraph() {
		let mut spec = Spec::default();

		{
			let (result, next_state) =
				Parser::<{ Section::Digraph as usize }>::next(&spec, "foobar").unwrap();

			assert_eq!(result, None);
			assert_eq!(
				next_state,
				State {
					word: "foobar",
					section: Section::Monograph,
				}
			);
		}

		spec.graphs.insert("foo", "@");
		{
			let (result, next_state) =
				Parser::<{ Section::Digraph as usize }>::next(&spec, "foobar").unwrap();

			assert_eq!(result, Some("@"));
			assert_eq!(
				next_state,
				State {
					word: "bar",
					section: Section::default(),
				}
			);
		}

		{
			let (result, next_state) =
				Parser::<{ Section::Digraph as usize }>::next(&spec, "qa").unwrap();

			assert_eq!(result, None);
			assert_eq!(
				next_state,
				State {
					word: "qa",
					section: Section::Monograph,
				}
			);
		}
	}

	#[test]
	fn test_sukuon() {
		let mut spec = Spec::default();

		{
			let (result, next_state) =
				Parser::<{ Section::Sukuon as usize }>::next(&spec, "ttest").unwrap();

			assert_eq!(result, None);
			assert_eq!(
				next_state,
				State {
					word: "ttest",
					section: Section::Digraph,
				}
			);
		}

		spec.graphemes.sukuon.graph = "@".into();
		spec.graphemes.sukuon.matches = {
			let mut m = HashSet::new();
			m.insert("tt");
			m
		};

		{
			let (result, next_state) =
				Parser::<{ Section::Sukuon as usize }>::next(&spec, "ttest").unwrap();

			assert_eq!(result, Some("@"));
			assert_eq!(
				next_state,
				State {
					word: "test",
					section: Section::default(),
				}
			);
		}

		{
			let (result, next_state) =
				Parser::<{ Section::Sukuon as usize }>::next(&spec, "x").unwrap();
			assert_eq!(result, None);
			assert_eq!(
				next_state,
				State {
					word: "x",
					section: Section::Nasal,
				}
			);
		}
	}

	#[test]
	fn test_monograph() {
		let mut spec = Spec::default();

		{
			let (result, next_state) =
				Parser::<{ Section::Monograph as usize }>::next(&spec, "mytest").unwrap();

			assert_eq!(result, None);
			assert_eq!(
				next_state,
				State {
					word: "mytest",
					section: Section::Nasal
				}
			);
		}

		spec.graphs.insert("my", "@");
		{
			let (result, next_state) =
				Parser::<{ Section::Monograph as usize }>::next(&spec, "mytest").unwrap();

			assert_eq!(result, Some("@"));
			assert_eq!(
				next_state,
				State {
					word: "test",
					section: Section::default(),
				}
			);
		}

		{
			let (result, next_state) =
				Parser::<{ Section::Monograph as usize }>::next(&spec, "x").unwrap();

			assert_eq!(result, None);
			assert_eq!(
				next_state,
				State {
					word: "x",
					section: Section::Nasal,
				}
			);
		}
	}

	#[test]
	fn test_nasal() {
		let mut spec = Spec::default();

		{
			let (result, next_state) =
				Parser::<{ Section::Nasal as usize }>::next(&spec, "atest").unwrap();

			assert_eq!(result, Some("a"));
			assert_eq!(
				next_state,
				State {
					word: "test",
					section: Section::default(),
				}
			);
		}

		spec.graphs.insert("a", "@");
		{
			let (result, next_state) =
				Parser::<{ Section::Nasal as usize }>::next(&spec, "atest").unwrap();

			assert_eq!(result, Some("@"));
			assert_eq!(
				next_state,
				State {
					word: "test",
					section: Section::default(),
				}
			);
		}

		{
			assert!(Parser::<{ Section::Nasal as usize }>::next(&spec, "").is_none());
		}
	}
}
