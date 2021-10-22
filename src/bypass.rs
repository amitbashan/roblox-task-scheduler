use std::ffi::CString;

use winapi::{
	shared::minwindef::LPVOID, um::{
		libloaderapi::{GetModuleHandleA, GetProcAddress},
		memoryapi::VirtualProtect,
		winnt::PAGE_EXECUTE_READWRITE,
	},
};

pub unsafe fn free_console() {
	const X86_RETURN_INSTRUCTION: u8 = 0xC3;

	let free_console_address = {
		let module_name = CString::new("kernel32.dll").unwrap();
		let function_name = CString::new("FreeConsole").unwrap();

		GetProcAddress(
			GetModuleHandleA(
				module_name.as_ptr()),
			function_name.as_ptr(),
		) as LPVOID
	};
	let mut old_protect = 0u32;

	VirtualProtect(
		free_console_address,
		std::mem::size_of_val(&X86_RETURN_INSTRUCTION),
		PAGE_EXECUTE_READWRITE,
		&mut old_protect,
	);

	std::ptr::write(free_console_address as *mut u8, X86_RETURN_INSTRUCTION);

	VirtualProtect(
		free_console_address,
		std::mem::size_of_val(&X86_RETURN_INSTRUCTION),
		old_protect,
		&mut old_protect,
	);
}
