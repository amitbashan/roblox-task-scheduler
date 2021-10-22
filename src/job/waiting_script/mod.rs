use crate::{job::Job, script_context::ScriptContext};

pub const NAME: &str = "WaitingScriptJob";
const SCRIPT_CONTEXT_OFFSET: isize = 76;

#[derive(Debug, Copy, Clone)]
pub struct WaitingScript {
	address: *const usize,
	pub script_context: ScriptContext,
}

impl WaitingScript {
	pub unsafe fn new(address: *const usize) -> Self {
		Self {
			address,
			script_context: ScriptContext::new(
				(
					*address.
						offset(SCRIPT_CONTEXT_OFFSET)
				) as *const usize
			),
		}
	}
}

impl Job for WaitingScript {
	fn address(&self) -> *const usize {
		self.address
	}
}
