# Kana
## About
Kana is a small CLI program for transliterating romaji text to either hiragana (ひらがな) or
katakana (カタカナ).

### How it works
Internally, it uses a finite-state machine in order to parse everything correctly and in the right
order.

## Example
```shell
$ kana ohayougozaimasu
おはようございます
$ kana --katakana suupaa mario
スーパー マリオ
```

## Building
Since this project is written in Rust, you need to build it with `cargo`:
```shell
$ cargo build --release --frozen
```

Shell completions are generated during build and stored in `target/completions` after it's finished.

## Note
Some few monographs are not implemented due to being redundant or generally unused.
