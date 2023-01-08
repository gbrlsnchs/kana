use std::{collections::HashMap, io::Result};

use default_macro::default;
use pretty_assertions::assert_eq;

use kana::{self, Config, Feature, SpecialChars};

#[test]
fn test_hiragana() -> Result<()> {
	let test_data = include_bytes!("data/hiragana.toml");
	let test_data: HashMap<&str, &str> = toml::from_slice(test_data)?;
	let test_data: Vec<(&str, &str, Config)> = test_data
		.iter()
		.map(|(k, v)| (*k, *v, default!(Config {})))
		.collect();

	assert_test_cases(test_data);

	Ok(())
}

#[test]
fn test_katakana() -> Result<()> {
	let test_data = include_bytes!("data/katakana.toml");
	let test_data: HashMap<&str, &str> = toml::from_slice(test_data)?;
	let test_data: Vec<(&str, &str, Config)> = test_data
		.iter()
		.map(|(k, v)| {
			(
				*k,
				*v,
				default!(Config {
					start_with_katakana: true
				}),
			)
		})
		.collect();

	assert_test_cases(test_data);

	Ok(())
}

#[test]
fn test_extended_katakana() -> Result<()> {
	let test_data = include_bytes!("data/extended_katakana.toml");
	let test_data: HashMap<&str, &str> = toml::from_slice(test_data)?;
	let test_data: Vec<(&str, &str, Config)> = test_data
		.iter()
		.map(|(k, v)| {
			(
				*k,
				*v,
				default!(Config {
					start_with_katakana: true,
					extended_katakana: true,
				}),
			)
		})
		.collect();

	assert_test_cases(test_data);

	Ok(())
}

#[test]
fn test_punctuation() -> Result<()> {
	let test_data = include_bytes!("data/punctuation.toml");
	let test_data: HashMap<&str, &str> = toml::from_slice(test_data)?;
	let test_data: Vec<(&str, &str, Config)> = test_data
		.iter()
		.map(|(k, v)| {
			(
				*k,
				*v,
				default!(Config {
					parse_punctuation: true,
				}),
			)
		})
		.collect();

	assert_test_cases(test_data);

	Ok(())
}

#[test]
fn test_inputs() {
	assert_test_cases(vec![
		("watashi", "わたし", default!(Config {})),
		(
			"gaburieru",
			"ガブリエル",
			default!(Config {
				start_with_katakana: true,
			}),
		),
		(
			"watashiha@gaburieru@desu",
			"わたしはガブリエルです",
			default!(Config {
				special_chars: {
					let mut chars = SpecialChars::new();
					chars.insert(Feature::Kana, '@');
					chars
				},
			}),
		),
		("HELLO", "へLLお", default!(Config {})),
		("hello", "へllお", default!(Config {})),
		(
			"#hello#",
			"hello",
			default!(Config {
				special_chars: {
					let mut chars = SpecialChars::new();
					chars.insert(Feature::RawText, '#');
					chars
				},
			}),
		),
		(
			"nihon@nihon@#Japan#nihon#@nihon@#",
			"にほんニホンJapanにほん@nihon@",
			default!(Config {
				special_chars: {
					let mut chars = SpecialChars::new();
					chars.insert(Feature::Kana, '@');
					chars.insert(Feature::RawText, '#');
					chars
				},
			}),
		),
		(
			"#rawtext",
			"rawtext",
			default!(Config {
				special_chars: {
					let mut chars = SpecialChars::new();
					chars.insert(Feature::RawText, '#');
					chars
				},
			}),
		),
		("", "", default!(Config {})),
		("'hana'", "'はな'", default!(Config {})),
		(
			"'hana'",
			"「はな」",
			default!(Config {
				parse_punctuation: true
			}),
		),
		(r#""onamae""#, r#""おなまえ""#, default!(Config {})),
		(
			r#""onamae""#,
			"『おなまえ』",
			default!(Config {
				parse_punctuation: true
			}),
		),
		(
			"chottomattekudasai.",
			"ちょっとまってください.",
			default!(Config {}),
		),
		(
			"chottomattekudasai.",
			"ちょっとまってください。",
			default!(Config {
				parse_punctuation: true
			}),
		),
		("chottomatte", "ちょっとまって", Config::default()),
		(
			"nikugazenzensukijaarimasen",
			"にくがぜんぜんすきじゃありません",
			default!(Config {}),
		),
		("wwwwwww", "wwwwwww", default!(Config {})),
		("日本", "日本", default!(Config {})),
		("nihon", "にほん", default!(Config {})),
		("12jinitabemasu!", "12じにたべます!", default!(Config {})),
		("123 GO!", "123 ご!", default!(Config {})),
		(
			"oomen",
			"オーメン",
			default!(Config {
				start_with_katakana: true
			}),
		),
		(
			"tsyuu",
			"ツュー",
			default!(Config {
				start_with_katakana: true,
				extended_katakana: true,
			}),
		),
		(
			"suupaamario",
			"スーパーマリオ",
			default!(Config {
				start_with_katakana: true,
			}),
		),
		(
			"pureisuteeshon",
			"プレイステーション",
			default!(Config {
				start_with_katakana: true,
			}),
		),
		(
			"monkii dii rufi",
			"モンキー ディー ルフィ",
			default!(Config {
				start_with_katakana: true,
				extended_katakana: true,
			}),
		),
		(
			"wwwwwww",
			"wwwwwww",
			default!(Config {
				start_with_katakana: true,
			}),
		),
		(
			"supagetti",
			"スパゲッティ",
			default!(Config {
				start_with_katakana: true,
				extended_katakana: true,
			}),
		),
		(
			"sandoicchi",
			"サンドイッチ",
			default!(Config {
				start_with_katakana: true,
			}),
		),
		(
			"egguheddo",
			"エッグヘッド",
			default!(Config {
				start_with_katakana: true,
			}),
		),
		(
			"日本",
			"日本",
			default!(Config {
				start_with_katakana: true,
			}),
		),
		(
			"123 GO!",
			"123 ゴ!",
			default!(Config {
				start_with_katakana: true,
			}),
		),
	]);
}

#[test]
fn test_pokemon() -> Result<()> {
	let test_data = include_bytes!("data/pokemon.toml");
	let test_data: HashMap<&str, &str> = toml::from_slice(test_data)?;
	let test_data: Vec<(&str, &str, Config)> = test_data
		.iter()
		.map(|(k, v)| {
			(
				*k,
				*v,
				default!(Config {
					start_with_katakana: true,
					extended_katakana: true,
					special_chars: {
						let mut chars = SpecialChars::new();
						chars.insert(Feature::Reset, '^');
						chars
					},
				}),
			)
		})
		.collect();

	assert_test_cases(test_data);

	Ok(())
}

fn assert_test_cases(cases: Vec<(&str, &str, Config)>) {
	for (input, expected, cfg) in cases {
		assert_eq!(
			kana::transliterate(input, &cfg),
			expected,
			"original input: {:?}",
			input
		);
	}
}
