kana(1)

# NAME

kana - Romaji to hiragana/katakana transliterator

# SYNOPSIS

*kana* [_OPTIONS_] [_INPUT_| < _input_]

# DESCRIPTION

*kana* is a CLI tool for converting romaji text into hiragana (ひらがな)
and katakana (カタカナ).

By default, it works with hiragana only. However, with the appropriate
flags and arguments, it also works with katakana and supports several extra
functionalities for it. See more details in the _EXTRA FEATURES_ section.

For quick usage, pass a romaji string as input:

	*kana <<< 'ohayougozaimasu'*

# OPTIONS

*-k*, *--katakana*
	Start parsing with katakana instead of hiragana.

*-e*, *--extended*
	Use an extended version of katakana.

*-p*, *--punctuation*
	Parse punctuation marks.

	If this option is not set, punctuation is printed as is.

*-P*, *--force-prolongation*
	Force hiragana to use a prolongation character.

*-t*, *--kana-toggle*=_CHAR_
	The character to be used in order to toggle kanas.

	When _CHAR_ is found in the input, kanas are toggled until _CHAR_
	if found again, an so on.

*-r*, *--raw-toggle*=_CHAR_
	The character to be used in order to toggle between raw text and kanas.

	When _CHAR_ is found in the input, raw text is printed istead of
	transliterated text until _CHAR_ is found again, and so on.

*-R*, *--prolongation-reset*=_CHAR_
	The character to be used in order to prevent a katakana prolongation
	to be printed.

	When a prolongation is to be printed, it checks for _CHAR_. If it
	is found in the input, no prolongation is printed, and the parser
	resets and then proceeds to parsing again.

*-s*, *--vowel-shortener*=_CHAR_
	The character to be used in order to print a small vowel.

	If _CHAR_ is found, a small variant of a vowel is printed. If _CHAR_
	is not a vowel, then the _CHAR_ is printed alongside whatever comes
	after it.

*-S*, *--virtual-stop*=_CHAR_
	The character to be used in order to print a glottal stop (a "sokuon",
	or 『そくおん』).

*-h*, *--help*
	Show help information.

	Note that -h shows a short help, while --help shows a long one.

*-v*, *--version*
	Show version.

# FEATURES

## EXTENDED KATAKANA

When using extended katakana, a set of extra syllabograms are taken into
account when parsing. This is particularly useful for loanwords, for example,
spaghetti:

	*kana --katakana --extended <<< 'supagetti'*

The command above outputs スパゲッティ, which contains extended
katakana syllabograms.

## PUNCTUATION MARKS

By default, *kana* doesn't transliterate punctuation marks. However, when using --with-punctuation,
it starts to take punctuation marks into account:

	*kana --punctuation <<< 'onamaeha? chottomatte! kawaii~,sugoidesune.'*

The command above outputs the following:

	おなまえは？ ちょっとまって！ かわいい〜、すごいですね。

Note that quoting also works accordingly:

	*kana --punctuation '"kirei"'"'benri'"*

Opening and closing quotes are automatically handled:

	『きれい』「べんり」

## FORCE PROLONGATION

By default, hiragana output doesn't use a prolongation character. Instead,
it simply repeats the prolongated vowel.

However, some words can benefit from showing the prolongation character even
for hiragana. It's possible to do so:

	*kana --force-prolongation <<< 'raamen'*

By forcing prolongation, the output looks like:

	らーめん

## KANA TOGGLING

This is a useful feature that allows toggling between kanas for a single run,
that is, for the same input. For example:

	*kana --kana-toggle='@' <<< 'watashiha@gaburieru@desu'*

The command above prints the following:

	わたしはガブリエルです

## RAW TEXT TOGGLING

Sometimes, mixing raw text is desired. By setting a toggle for raw text,
it's possible to accomplish that:

	*kana --raw-toggle='#' <<< 'watashiha#J-rock#gasukidesu'*

The resulting output for the command above is:

	わたしはJ-rockがすきです

## PROLONGATION RESET

When parsing katakana, *kana* automatically prints a prolongation for repeated
vowels. However, sometimes, this is not desired. It's possible to reset the prolongation:

	*kana --katakana --prolongation-reset='^' Pikachu^u*

Now you can correctly print some Pokémon names, like Pikachu's, whose output is:

	ピカチュウ

## SMALL VOWEL

Some words, for some reason, are not prolonged by the chouonpu
(ちょうおんぷ) mark, but by a small vowel instead.

This is notoriously true for Pokémon names, for example:

	*kana --katakana --vowel-shortener='_' <<< 'Serebi_i'*

The command above renders the following output:

	セレビィ

Note that it also works with the prolongation mark:

	*kana --katakana --vowel-shortener='_' <<< 'Me_eekuru'*

The output for the above:

	メェークル

## VIRTUAL STOP

This is a niche feature, mostly useful for some loanwords (probably Pokémon
names again). It allows freely inserting a glottal stop mark:

	*kana --katakana --virtual-stop='%' <<< 'U%u'*

The output for the above is:

	ウッウ

# AUTHORS

Developed and maintained by Gabriel Sanches <gabriel@gsr.dev>.

Source code is located at <https://git.sr.ht/~gbrlsnchs/kana>.
