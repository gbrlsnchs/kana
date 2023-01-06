pub fn count_chars(s: &str) -> usize {
	s.chars().count()
}

pub fn slice_from(s: &str, n: usize) -> &str {
	&s[size(s, n)..]
}

pub fn slice_to(s: &str, n: usize) -> &str {
	&s[..size(s, n)]
}

fn size(s: &str, n: usize) -> usize {
	s.chars().take(n).map(|c| c.len_utf8()).sum()
}
