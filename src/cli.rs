use clap::Parser;

#[derive(Default, Parser)]
#[clap(name = "kana")]
#[clap(about = "A romaji to hiragana/katakana literal converter")]
#[clap(author, version, about, long_about = None)]
pub struct Args {
	#[clap(long, short, help = "Transliterate to Katakana")]
	pub katakana: bool,

	#[clap(
		value_parser,
		required = true,
		help = "List of words to be transliterated"
	)]
	pub words: Vec<String>,
}
