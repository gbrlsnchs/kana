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
//! assert_eq!(result, "おはようございます");
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
//! assert_eq!(result, "エルデンリング");
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
//! assert_eq!(result, "スパゲッティ");
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
//! assert_eq!(result, "わたしはガブリエルです");
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
//! assert_eq!(result, "わたしはGabrielです");
//! ```
//!
//! When using katakana, it is possible to do some more complex things, like:
//! - Reset prolongations
//! - Add a small vowel instead of prolongation
//! - Add a prolongation after a small vowel
//! - Add a virtual glottal stop
//!
//! ```
//! use kana::{Config, CharFeature, SpecialChars};
//!
//! let cfg = Config{
//!     start_with_katakana: true,
//!     special_chars: {
//!         let mut s = SpecialChars::new();
//!         s.insert(CharFeature::ResetProlongation, '^');
//!         s.insert(CharFeature::SmallVowel, '_');
//!         s.insert(CharFeature::VirtualStop, '%');
//!         s
//!     },
//!     ..Config::default()
//! };
//!
//! let reset_prolongation_result = kana::transliterate("Pikachu^u", &cfg);
//! assert_eq!(reset_prolongation_result, "ピカチュウ");
//!
//! let small_vowel_result = kana::transliterate("Serebi_i", &cfg);
//! assert_eq!(small_vowel_result , "セレビィ");
//!
//! let small_vowel_prolongation_result = kana::transliterate("Me_eekuru", &cfg);
//! assert_eq!(small_vowel_prolongation_result, "メェークル");
//!
//! let virtual_stop_result = kana::transliterate("U%u", &cfg);
//! assert_eq!(virtual_stop_result, "ウッウ");
//! ```
//!
//! It is possible to reset the prolongation in katakana words:
//!
//! ```
//! use kana::{Config, CharFeature, SpecialChars};
//!
//! let cfg = Config{
//!     start_with_katakana: true,
//!     special_chars: {
//!         let mut s = SpecialChars::new();
//!         s.insert(CharFeature::ResetProlongation, '^');
//!         s
//!     },
//!     ..Config::default()
//! };
//!
//! let result = kana::transliterate("Pikachu^u", &cfg);
//! assert_eq!(result, "ピカチュウ", "prolongation reset example");
//! ```

mod parser;
mod transliterate;

pub use transliterate::*;
