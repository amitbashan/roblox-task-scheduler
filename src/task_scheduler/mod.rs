use crate::job::{*, generic::Generic, waiting_script::WaitingScript};

mod offset;

pub struct TaskScheduler {
	pub address: *const usize,
	pub jobs: Vec<Kind>,
}

impl TaskScheduler {
	pub unsafe fn new(address: *const usize) -> Self {
		Self {
			address,
			jobs: {
				let start = *address.offset(offset::jobs::START);
				let end = *address.offset(offset::jobs::END);

				(start..end)
					.step_by(2 * std::mem::size_of::<*const *const usize>())
					.map(|index| {
						let job = Generic::new(*(index as *const *const usize));

						match job.name() {
							waiting_script::NAME => Kind::WaitingScript(
								WaitingScript::new(job.address())
							),
							_ => Kind::Generic(job),
						}
					})
					.collect()
			},
		}
	}
}
