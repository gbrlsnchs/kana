pub(super) fn word_count(word: &str) -> usize {
	word.chars().count()
}

pub(super) fn word_slice_from(word: &str, n: usize) -> &str {
	let size = word_slice_size(word, n);

	&word[size..]
}

pub(super) fn word_slice_until(word: &str, n: usize) -> &str {
	let size = word_slice_size(word, n);

	&word[..size]
}

fn word_slice_size(word: &str, n: usize) -> usize {
	word.chars().take(n).map(|c| c.len_utf8()).sum()
}
