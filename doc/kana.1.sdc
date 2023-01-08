kana(1)

# NAME

kana - Romaji to hiragana/katakana transliterator

# SYNOPSIS

*kana* [_OPTIONS_] [_INPUT_| < _input_]

# DESCRIPTION

*kana* is a CLI tool for converting romaji text into hiragana/katakana.

By default, it works with hiragana only. However, with the appropriate
flags and arguments, it also works with katakana and supports several extra
functionalities for it. See more details in the _EXTRA FEATURES_ section.

For quick usage, simply pass the input as arguments:

	*kana ohayougozaimasu*

It's also possible to take input from _stdin_:

	*kana --interactive < input*

# OPTIONS

*-i*, *--interactive*
	Read input from _stdin_.

*-k*, *--katakana*
	Start parsing with katakana instead of hiragana.

*-p*, *--with-punctuation*
	Take punctuation marks into account.

	If this option is not set, punctuation is printed as is.

*-t*, *--kana-toggle*=_CHAR_
	The character to be used in order to toggle kanas.

	When _CHAR_ is found in the input, kanas are toggled until _CHAR_
	if found again, an so on.

*-r*, *--raw-text-toggle*=_CHAR_
	The character to be used in order to toggle between raw text and kanas.

	When _CHAR_ is found in the input, raw text is printed istead of
	transliterated text until _CHAR_ is found again, and so on.

*-R*, *--prolongation-reset-char*=_CHAR_
	The character to be used in order to prevent a katakana prolongation
	to be printed.

	When a prolongation is to be printed, it checks for _CHAR_. If it
	is found in the input, no prolongation is printed, and the parser
	resets and then proceeds to parsing again. Only works with katakana.

*-s*, *--small-vowel-char*=_CHAR_
	The character to be used in order to print a small vowel.

	If _CHAR_ is found, a small variant of a vowel is printed. If _CHAR_
	is not a vowel, then the _CHAR_ is printed alongside whatever comes
	after it. Only works with katakana.

*-S*, *--virtual-stop-char*=_CHAR_
	The character to be used in order to print a glottal stop (a "sokuon",
	or 『そくおん』).

*-h*, *--help*
	Show help information.

	Note that -h shows a short help, while --help shows a long one.

*-v*, *--version*
	Show version.

# EXTRA FEATURES

## EXTENDED KATAKANA

When using extended katakana, a set of extra syllabograms are taken into
account when parsing. This is particularly useful for loanwords, for example,
spaghetti:

	*kana --katakana --extended-katakana supagetti*

The command above outputs スパゲッティ, which contains extended
katakana syllabograms.

## PUNCTUATION MARKS

By default, *kana* doesn't transliterate punctuation marks. However, when using --with-punctuation,
it starts to take punctuation marks into account:

	*kana --with-punctuation 'onamaeha?' 'chottomatte!' 'kawaii~' 'sugoidesune.'*

The command above outputs the following:

	おなまえは？ ちょっとまって！ かわいい〜 すごいですね。

## KANA TOGGLING

This is a useful feature that allows toggling between kanas for a single run,
that is, for the same input. For example:

	*kana --kana-toggle=@ watashiha@gaburieru@desu*

The command above prints the following:

	わたしはガブリエルです

## RAW TEXT TOGGLING

Sometimes, mixing raw text is desired. By setting a toggle for raw text,
it's possible to accomplish that:

	*kana --raw-text-toggle=# watashiha#J-rock#gasukidesu*

The resulting output for the command above is:

	わたしはJ-rockがすきです

## PROLONGATION RESET (KATAKANA ONLY)

When parsing katakana, *kana* automatically prints a prolongation for repeated
vowels. However, sometimes, this is not desired. It's possible to reset the prolongation:

	*kana --katakana --prolongation-reset-char=^ Pikachu^u*

Now you can correctly print some Pokémon names, like Pikachu's, whose output is:

	ピカチュウ

## SMALL VOWEL (KATAKANA ONLY)

Some words, for some reason, are not prolonged by the chouonpu
(ちょうおんぷ) mark, but by a small vowel instead.

This is notoriously true for Pokémon names, for example:

	*kana --katakana --small-vowel-char=\_ Serebi_i*

The command above renders the following output:

	セレビィ

Note that it also works with the prolongation mark:

	*kana --katakana --small-vowel-char=\_ Me_eekuru*

The output for the above:

	メェークル

## VIRTUAL STOP

This is a niche feature, mostly useful for some loanwords (probably Pokémon
names again). It allows freely inserting a glottal stop mark:

	*kana --katakana --virtual-stop-char=% U%u*

The output for the above is:

	ウッウ

# SEE ALSO

The *kana* engine has a web interface at <https://kana.guru>.

# AUTHORS

Developed and maintained by Gabriel Sanches <gabriel@gsr.dev>.

Source code is located at <https://git.sr.ht/~gbrlsnchs/kana>.
