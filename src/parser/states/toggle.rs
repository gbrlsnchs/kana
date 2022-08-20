pub const SIZE: usize = 1;

pub enum Type {
	Kana = 1,
}

#[derive(Debug, PartialEq)]
pub struct Toggle<'a, const TYPE: usize>(pub &'a str, pub Option<char>, pub bool);
