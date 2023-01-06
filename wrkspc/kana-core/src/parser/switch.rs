pub struct Switch<T> {
	data: [T; 2],
	index: usize,
}

impl<T> Switch<T> {
	pub fn new(data: [T; 2]) -> Self {
		Switch { data, index: 0 }
	}

	pub fn get_current(&self) -> &T {
		&self.data[self.index]
	}

	pub fn toggle(&mut self) {
		self.index = 1 - self.index;
	}
}
