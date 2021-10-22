mod offset;

#[derive(Debug, Copy, Clone)]
pub struct ScriptContext {
	pub address: *const usize,
	pub luau_state: *const usize,
}

impl ScriptContext {
	pub unsafe fn new(address: *const usize) -> Self {
		Self {
			address,
			luau_state: (address.offset(offset::LUAU_STATE) as isize
				- (*(address.offset(offset::LUAU_STATE)) as isize)) as *const usize,
		}
	}
}
