use clap::Parser;

#[derive(Default, Parser)]
#[clap(name = "kana")]
#[clap(about = r#"
Kana is a CLI program for transliterating romaji text to either hiragana or katakana.

Under the hood, it uses a finite-state machine in order to parse the text correctly. It tries to
implement all standard syllables for both kanas. Most extended syllables for katakana are also
implemented, being left out only ones that are either obsolete or redundant and older.
"#)]
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
