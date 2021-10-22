use cxx::CxxString;

pub mod offset;
pub mod generic;
pub mod waiting_script;

pub trait Job {
	fn address(&self) -> *const usize;

	unsafe fn name(&self) -> &str {
		std::mem::transmute::<_, &CxxString>(
			self.address().offset(offset::NAME)
		).to_str().unwrap()
	}

	unsafe fn time_elapsed(&self) -> &f64 {
		&*(self.address().offset(offset::TIME_ELAPSED) as *const f64)
	}
}

pub enum Kind {
	Generic(Box<dyn Job>),
	WaitingScript(waiting_script::WaitingScript),
}

impl Job for Kind {
	fn address(&self) -> *const usize {
		match self {
			Kind::Generic(job) => job.address(),
			Kind::WaitingScript(job) => job.address(),
		}
	}
}
