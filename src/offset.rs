use lazy_static::*;
use winapi::um::libloaderapi::GetModuleHandleA;

pub const GET_TASK_SCHEDULER: usize = 0xDDCEE0;

lazy_static! {
	static ref BASE: usize = unsafe { GetModuleHandleA(std::ptr::null()) as usize };
}

pub fn rebase(offset: usize) -> usize {
	offset + *BASE
}
