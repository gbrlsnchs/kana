//! kana is a library for transliterating romaji text to hiragana or katakana.
//!
//! It supports some extra, optional features:
//! - Using an extended version of katakana
//! - Toggling between kanas and also raw text during transliteration
//! - Parsing punctuation marks
//!
//! By default, every feature is turned off, and only hiragana is transliterated:
//!
//! ```
//! use kana::Config;
//!
//! let cfg = Config::default();
//! let result = kana::transliterate("ohayougozaimasu", &cfg);
//!
//! assert_eq!(result, "おはようございます", "default example");
//! ```
//!
//! It's possible to output katakana instead of hiragana:
//!
//! ```
//! use kana::Config;
//!
//! let cfg = Config{
//!     start_with_katakana: true,
//!     ..Config::default()
//! };
//! let result = kana::transliterate("erudenringu", &cfg);
//!
//! assert_eq!(result, "エルデンリング", "katakana first example");
//! ```
//!
//! Use extended katakana to get more combinations for the result:
//! ```
//! use kana::Config;
//!
//! let cfg = Config{
//!     start_with_katakana: true,
//!     extended_katakana: true,
//!     ..Config::default()
//! };
//! let result = kana::transliterate("supagetti", &cfg);
//!
//! assert_eq!(result, "スパゲッティ", "extended katakana example");
//! ```
//!
//! A toggle character can be passed via configuration in order to toggle
//! between kanas everytime the toggle is matched:
//!
//! ```
//! use kana::{Config, CharFeature, SpecialChars};
//!
//! let cfg = Config{
//!     special_chars: {
//!         let mut s = SpecialChars::new();
//!         s.insert(CharFeature::KanaToggle, '@');
//!         s
//!     },
//!     ..Config::default()
//! };
//! let result = kana::transliterate("watashiha@gaburieru@desu", &cfg);
//!
//! assert_eq!(result, "わたしはガブリエルです", "kana toggle example");
//! ```
//!
//! Another toggle can also be passed via configuration for toggling between
//! default parsing and raw text:
//!
//! ```
//! use kana::{Config, CharFeature, SpecialChars};
//!
//! let cfg = Config{
//!     special_chars: {
//!         let mut s = SpecialChars::new();
//!         s.insert(CharFeature::RawTextToggle, '#');
//!         s
//!     },
//!     ..Config::default()
//! };
//!
//! let result = kana::transliterate("watashiha#Gabriel#desu", &cfg);
//! assert_eq!(result, "わたしはGabrielです", "raw text toggle example");
//! ```

mod parser;
mod transliterate;

pub use transliterate::*;
