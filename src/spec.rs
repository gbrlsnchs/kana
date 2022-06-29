use std::collections::{HashMap, HashSet};

use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct Spec<'a> {
	#[serde(borrow)]
	pub graphs: HashMap<&'a str, &'a str>,
	pub graphemes: Graphemes<'a>,
}

#[derive(Debug, Default, Deserialize)]
pub struct Graphemes<'a> {
	#[serde(borrow)]
	pub sukuon: Grapheme<'a>,
	pub choonpu: Option<Grapheme<'a>>,
}

#[derive(Debug, Default, Deserialize)]
pub struct Grapheme<'a> {
	#[serde(borrow)]
	pub matches: HashSet<&'a str>,
	pub graph: &'a str,
}
