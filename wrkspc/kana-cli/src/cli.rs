use clap::{ArgAction, Parser};

/// kana is a CLI tool for transliterating romaji text to hiragana (ひらがな) and katakana
/// (カタカナ).
///
/// See kana(1) for more details about usage.
#[derive(Default, Parser)]
#[command(
	about,
	long_about,
	version,
	max_term_width = 80,
	disable_help_flag = true,
	disable_version_flag = true
)]
pub struct Kana {
	/// Whether to read romaji input from stdin.
	#[arg(long, short)]
	pub interactive: bool,

	/// Start parsing with katakana instead of hiragana.
	#[arg(long, short)]
	pub katakana: bool,

	/// Use extended katakana.
	#[arg(long, short)]
	pub extended_katakana: bool,

	/// Parse punctuation marks.
	#[arg(long, short = 'p')]
	pub with_punctuation: bool,

	/// Use a character to toggle between kanas.
	#[arg(long, short = 't')]
	pub kana_toggle: Option<char>,

	/// Use a character to toggle between raw text and kanas.
	#[arg(long, short)]
	pub raw_text_toggle: Option<char>,

	/// Use a character to reset prolongations when using katakana.
	#[arg(long, short = 'R')]
	pub prolongation_reset_char: Option<char>,

	/// Use a character to insert a small vowel when using katakana.
	#[arg(long, short)]
	pub small_vowel_char: Option<char>,

	/// Use a character to insert a virtual glottal stop.
	#[arg(long, short = 'S')]
	pub virtual_stop_char: Option<char>,

	/// Show help information.
	#[arg(long, short, action = ArgAction::Help)]
	pub help: Option<bool>,

	/// Show version.
	#[arg(long, short, action = ArgAction::Version)]
	pub version: Option<bool>,

	/// Romaji input.
	#[arg()]
	pub input: Option<Vec<String>>,
}
