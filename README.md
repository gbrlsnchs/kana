# Kana
## About
Kana is a small CLI program for transliterating romaji text to either hiragana (ひらがな) or
katakana (カタカナ).

### How it works
Internally, it uses a finite-state machine in order to parse everything correctly and in the right
order.

## Examples
### Simple usage
```shell
$ kana ohayougozaimasu
おはようございます
$ kana --katakana suupaa mario
スーパー マリオ
```

### Toggling between kanas
It's possible to toggle between kanas if a toggle char is informed to `--toggle-char`. Whenever the
parser hits the informed char, it will toggle between kanas, using the other variant until it hits
that char again, then toggling back to the previous variant, and so on:
```shell
$ kana --toggle-char=@ watashiha@gaburieru@desu
わたしはガブリエルです
```

### Transliterating a multi-line file
Kana can't read files, so it is necessary to pipe files through something like `xargs`:
```shell
$ cat file.txt
hajimemashite
@gaburieru@tomoshimasu
douzoyoroshikuonegaishimasu

$ cat file.txt | xargs --max-lines=1 kana
はじめまして
ガブリエルともします
どうぞよろしくおねがいします
```

## Building from source
Since this project is written in Rust, you need to build it with `cargo`:
```shell
$ cargo build --release --frozen
```

Shell completions are generated during build and stored in `target/completions` after it's finished.

## Note
Some few monographs are not implemented due to being redundant or generally unused.
