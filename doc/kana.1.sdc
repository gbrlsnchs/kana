kana(1)

# NAME

kana - Romaji to hiragana/katakana transliterator

# SYNOPSIS

*kana* [_OPTIONS_...] < _input_

# DESCRIPTION

*kana* is a CLI tool for converting romaji text into hiragana (ひらがな)
and katakana (カタカナ).

By default, it works with hiragana only. However, with the appropriate
flags and arguments, it also works with katakana and supports several extra
functionalities for it. See more details in the _EXTRA FEATURES_ section.

For quick usage, pass a romaji string as input:

	*kana <<< 'ohayougozaimasu'*

# OPTIONS

*-k*
	Start parsing with katakana instead of hiragana.

*-x*
	Use an extended version of katakana.

*-pun*
	Parse punctuation marks.

	If this option is not set, punctuation is printed as is.

*-pro*
	Force hiragana to use a prolongation character.

*-t*=_CHAR_
	The character to be used in order to toggle kanas.

	When _CHAR_ is found in the input, kanas are toggled until _CHAR_
	if found again, an so on.

*-raw*=_CHAR_
	The character to be used in order to toggle between raw text and kanas.

	When _CHAR_ is found in the input, raw text is printed istead of
	transliterated text until _CHAR_ is found again, and so on.

*-res*=_CHAR_
	The character to be used in order to prevent a katakana prolongation
	to be printed.

	When a prolongation is to be printed, it checks for _CHAR_. If it
	is found in the input, no prolongation is printed, and the parser
	resets and then proceeds to parsing again.

*-vow*=_CHAR_
	The character to be used in order to print a small vowel.

	If _CHAR_ is found, a small variant of a vowel is printed. If _CHAR_
	is not a vowel, then the _CHAR_ is printed alongside whatever comes
	after it.

*-stop*=_CHAR_
	The character to be used in order to print a glottal stop (a "sokuon",
	or 『そくおん』).

*-h*
	Show help information.

*-version*
	Show version.

# FEATURES

## Extended katakana

When using extended katakana, a set of extra syllabograms are taken into
account when parsing. This is particularly useful for loanwords, for example,
spaghetti:

	*kana -k -x <<< 'supagetti'*

The command above outputs スパゲッティ, which contains extended
katakana syllabograms.

## Punctuation marks

By default, *kana* doesn't transliterate punctuation marks. However, when
using -pun, it starts to take punctuation marks into account:

	*kana -pun <<< 'onamaeha? chottomatte! kawaii~,sugoidesune.'*

The command above outputs the following:

	おなまえは？ ちょっとまって！ かわいい〜、すごいですね。

Note that quoting also works accordingly:

	*kana -pun <<< '"kirei"'"'benri'"*

Opening and closing quotes are automatically handled:

	『きれい』「べんり」

## Force prolongation

By default, hiragana output doesn't use a prolongation character. Instead,
it simply repeats the prolongated vowel.

However, some words can benefit from showing the prolongation character even
for hiragana. It's possible to do so:

	*kana -pro <<< 'raamen'*

By forcing prolongation, the output looks like:

	らーめん

## Kana toggling

This is a useful feature that allows toggling between kanas for a single run,
that is, for the same input. For example:

	*kana -t=@ <<< 'watashiha@gaburieru@desu'*

The command above prints the following:

	わたしはガブリエルです

## Raw text toggling

Sometimes, mixing raw text is desired. By setting a toggle for raw text,
it's possible to accomplish that:

	*kana -raw=# <<< 'watashiha#J-rock#gasukidesu'*

The resulting output for the command above is:

	わたしはJ-rockがすきです

## Prolongation reset

When parsing katakana, *kana* automatically prints a prolongation for
repeated vowels. However, sometimes, this is not desired. It's possible to
reset the prolongation:

	*kana -k -res=^ <<< 'Pikachu^u'*

Now you can correctly print some Pokémon names, like Pikachu's, whose output is:

	ピカチュウ

## Small vowel

Some words, for some reason, are not prolonged by the chouonpu
(ちょうおんぷ) mark, but by a small vowel instead.

This is notoriously true for Pokémon names, for example:

	*kana -k -vow=\_ <<< 'Serebi_i'*

The command above renders the following output:

	セレビィ

Note that it also works with the prolongation mark:

	*kana -k -s=\_ <<< 'Me_eekuru'*

The output for the above:

	メェークル

## Virtual stop

This is a niche feature, mostly useful for some loanwords (probably Pokémon
names again). It allows freely inserting a glottal stop mark:

	*kana -k -stop=% <<< 'U%u'*

The output for the above is:

	ウッウ

# AUTHORS

Developed and maintained by Gabriel Sanches <gabriel@gsr.dev>.

Source code is located at <https://sr.ht/~gbrlsnchs/kana>.
