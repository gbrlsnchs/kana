## kana

kana is a CLI tool and also a library for transliterating romaji text to either hiragana
(ひらがな) or katakana (カタカナ).

### Usage

The following snippets only shortly demonstrate the features. For a more thorough documentation,
see `kana(1)`.

#### Default features

```console
$ kana ohayougozaimasu
おはようございます
```

#### Reading from stdin

```console
$ echo "itadakimasu" | kana --interactive
いただきます
```

#### Katakana

```console
$ kana --katakana suupa mario
スーパ マリオ
```

#### Extended katakana


```console
$ kana --katakana --extended-katakana supagetti
スパゲッティ
```

#### Punctuation marks

```console
$ kana --with-punctuation "([{<soudesune." "'sugoi!'" '"kawaii~">}])' "?!,"
（［｛【そうですね。 「すごい！」 『かわいい〜』】｝］）？！、
```

#### Forcing prolongation

```console
$ kana --force-prolongation raamen
らーめん
```

#### Toggling between kanas

```console
$ kana --kana-toggle="@" watashiha@gaburieru@desu
わたしはガブリエルです
```

#### Toggling between raw text and kanas

```console
$ kana --raw-text-toggle="#" watashiha#J-rock#gasukidesu
わたしはJ-rockがすきです
```

#### Resetting prolongation (katakana only)

```console
$ kana --katakana --prolongation-reset-char="^" Pikachu^u
ピカチュウ
```

#### Using small vowels (katakana only)

```console
$ kana --katakana --small-vowel-char="_" Serebi_i
セレビィ
```

#### Adding virtual stops

```console
$ kana --katakana --virtual-stop-char="%" U%u
ウッウ
```

### Contributing

[Use the mailing list](mailto:~gbrlsnchs/kana-dev@lists.sr.ht) to
- Report issues
- Request new features
- Send patches
- Discuss development in general

If applicable, a new ticket will be submitted by maintainers to [the issue
tracker](https://todo.sr.ht/~gbrlsnchs/kana) in order to track confirmed bugs or new features.

### Building and distributing the project

This project is built entirely in Rust. Build it as you wish for local usage, and package it
to your distro of preference in accordance with its policy on how to package Rust projects.

> **_NOTE:_** `cargo build` generates shell completions for Bash, ZSH and Fish, which
> are available at `target/completions`, and manpages at `target/doc` (only when
> [`scdoc`](https://git.sr.ht/~sircmpwn/scdoc) is available).
