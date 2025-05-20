use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct ToastID(usize);

impl ToastID {
	pub fn new() -> Self {
		Self(100000)
	}

	pub fn add(&mut self) -> usize {
		let current = self.0;
		if self.0 == usize::MAX {
			self.0 = 100000;
		} else {
			self.0 += 1;
		}

		current
	}
}

impl Default for ToastID {
	fn default() -> Self {
		Self::new()
	}
}

impl Display for ToastID {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("{}", self.0))
	}
}
