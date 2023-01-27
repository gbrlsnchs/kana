## kana

kana is a CLI tool and also a library for transliterating romaji text to either hiragana
(ひらがな) or katakana (カタカナ).

### Usage

The following snippets only shortly demonstrate the features. For a more thorough documentation,
see `kana(1)`.

```shell
$ kana <<< 'ohayougozaimasu'
おはようございます
```

#### Katakana

```shell
$ kana --katakana <<< 'suupa mario'
スーパ マリオ
```

#### Extended katakana


```shell
$ kana --katakana --extended <<< 'supagetti'
スパゲッティ
```

#### Punctuation marks

```shell
$ kana --punctuation <<< 'soudesune.'
そうですね。
$ kana --punctuation <<< 'kawaii~'
かわいい〜
$ kana --punctuation <<< 'nani?!'
なに？！
$ kana --punctuation <<< "'hiragana'"
「ひらがな」
$ kana --punctuation <<< '"katakana"'
『カタカナ』
$ kana --punctuation <<< '([{<sugoi>}])'
（［｛【すごい】｝］）
```

#### Forcing prolongation (hiragana only)

```shell
$ kana --force-prolongation <<< 'raamen'
らーめん
```

#### Toggling between kanas

```shell
$ kana --kana-toggle='@' <<< 'watashiha@gaburieru@desu'
わたしはガブリエルです
```

#### Toggling between raw text and kanas

```shell
$ kana --raw-toggle='#' <<< 'watashiha#J-rock#gasukidesu'
わたしはJ-rockがすきです
```

#### Resetting prolongation

```shell
$ kana --katakana --prolongation-reset='^' <<< 'Pikachu^u'
ピカチュウ
```

#### Using small vowels

```shell
$ kana --katakana --vowel-shortener='_' <<< 'Keeshi_i'
ケーシィ
```

#### Adding virtual stops

```shell
$ kana --katakana --virtual-stop='%' <<< 'U%u'
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

This project is built entirely in Zig. Build it as you wish for local usage, and package it
to your distro of preference in accordance with its policy on how to package Zig projects.

You can generate man pages in `doc/` by using [`scdoc`](https://git.sr.ht/~sircmpwn/scdoc).
