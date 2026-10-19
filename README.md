## kana

kana is a CLI tool and also a library for transliterating romaji text to either hiragana
(ひらがな) or katakana (カタカナ).

### Usage

The following snippets only shortly demonstrate the features. For a more thorough documentation,
see `kana(1)`.

```
$ kana <<< 'ohayougozaimasu'
おはようございます
```

#### Katakana

```
$ kana -k <<< 'suupa mario'
スーパ マリオ
```

#### Extended katakana


```
$ kana -k -x <<< 'supagetti'
スパゲッティ
```

#### Punctuation marks

```
$ kana -pun <<< 'soudesune.'
そうですね。
$ kana -pun <<< 'kawaii~'
かわいい〜
$ kana -pun <<< 'nani?!'
なに？！
$ kana -pun <<< "'hiragana'"
「ひらがな」
$ kana -pun <<< '"katakana"'
『カタカナ』
$ kana -pun <<< '([{<sugoi>}])'
（［｛【すごい】｝］）
```

#### Forcing prolongation (hiragana only)

```
$ kana -pro <<< 'raamen'
らーめん
```

#### Toggling between kanas

```
$ kana -t=@ <<< 'watashiha@gaburieru@desu'
わたしはガブリエルです
```

#### Toggling between raw text and kanas

```
$ kana -raw=# <<< 'watashiha#J-rock#gasukidesu'
わたしはJ-rockがすきです
```

#### Resetting prolongation

```
$ kana -k -res=^ <<< 'Pikachu^u'
ピカチュウ
```

#### Using small vowels

```
$ kana -k -vow=_ <<< 'Keeshi_i'
ケーシィ
```

#### Adding virtual stops

```
$ kana -k -stop=% <<< 'U%u'
ウッウ
```

### Building

- [Zig](https://ziglang.org/download/) 0.16
- [scdoc](https://git.sr.ht/~sircmpwn/scdoc) (for man pages)

### Contributing

[Pick the appropriate mailing list](https://sr.ht/~gbrlsnchs/kana/lists) and send an email.

When a bug gets confirmed, a new ticket will be submitted by maintainers to [the issue
tracker](https://todo.sr.ht/~gbrlsnchs/kana), so folks can also follow the progress there.
