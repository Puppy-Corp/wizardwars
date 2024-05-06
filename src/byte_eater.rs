pub struct ByteEater<'a> {
	bytes: &'a [u8],
	index: usize,
}

impl<'a> ByteEater<'a> {
	pub fn new(bytes: &'a [u8]) -> Self {
		ByteEater {
			bytes,
			index: 0,
		}
	}

	pub fn read_u8(&mut self) -> u8 {
		let value = self.bytes[self.index];
		self.index += 1;
		value
	}

	pub fn read_u16(&mut self) -> u16 {
		let value = u16::from_le_bytes([
			self.bytes[self.index],
			self.bytes[self.index + 1],
		]);
		self.index += 2;
		value
	}

	pub fn read_u32(&mut self) -> u32 {
		let value = u32::from_le_bytes([
			self.bytes[self.index],
			self.bytes[self.index + 1],
			self.bytes[self.index + 2],
			self.bytes[self.index + 3],
		]);
		self.index += 4;
		value
	}

	pub fn read_u64(&mut self) -> u64 {
		let value = u64::from_le_bytes([
			self.bytes[self.index],
			self.bytes[self.index + 1],
			self.bytes[self.index + 2],
			self.bytes[self.index + 3],
			self.bytes[self.index + 4],
			self.bytes[self.index + 5],
			self.bytes[self.index + 6],
			self.bytes[self.index + 7],
		]);
		self.index += 8;
		value
	}

	pub fn read_f32(&mut self) -> f32 {
		let value = f32::from_le_bytes([
			self.bytes[self.index],
			self.bytes[self.index + 1],
			self.bytes[self.index + 2],
			self.bytes[self.index + 3],
		]);
		self.index += 4;
		value
	}

	pub fn read_f64(&mut self) -> f64 {
		let value = f64::from_le_bytes([
			self.bytes[self.index],
			self.bytes[self.index + 1],
			self.bytes[self.index + 2],
			self.bytes[self.index + 3],
			self.bytes[self.index + 4],
			self.bytes[self.index + 5],
			self.bytes[self.index + 6],
			self.bytes[self.index + 7],
		]);
		self.index += 8;
		value
	}

	pub fn read_bytes(&mut self, len: usize) -> &'a [u8] {
		let start = self.index;
		self.index += len;
		&self.bytes[start..self.index]
	}

	pub fn set_index(&mut self, index: usize) {
		self.index = index;
	}

	pub fn index(&self) -> usize {
		self.index
	}
}
