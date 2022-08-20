#[derive(Debug, PartialEq)]
pub struct Syllabogram<'a, const SIZE: usize>(pub &'a str, pub Option<char>);
