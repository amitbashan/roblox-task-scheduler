use crate::job::Job;

pub struct Generic {
	address: *const usize,
}

impl Generic {
	pub fn new(address: *const usize) -> Box<dyn Job> {
		Box::new(
			Self {
				address
			}
		) as Box<dyn Job>
	}
}

impl Job for Generic {
	fn address(&self) -> *const usize {
		self.address
	}
}
