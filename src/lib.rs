use lazy_static::*;
use winapi::{
	shared::minwindef::{BOOL, HINSTANCE, LPVOID, TRUE},
	um::{consoleapi, winnt},
};

use crate::job::Kind;

mod offset;
mod bypass;
mod job;
mod script_context;
mod task_scheduler;

lazy_static! {
	static ref GET_TASK_SCHEDULER: fn() -> *const usize = unsafe { std::mem::transmute(offset::rebase(offset::GET_TASK_SCHEDULER)) };
}

fn main() {
	unsafe {
		bypass::free_console();
		consoleapi::AllocConsole();

		let task_scheduler = task_scheduler::TaskScheduler::new(GET_TASK_SCHEDULER());

		for job in task_scheduler.jobs {
			match job {
				Kind::Generic(job) => println!("Generic job: {}", job.name()),
				Kind::WaitingScript(job) => println!("Script Context: {:#X?}", job.script_context),
			}
		}
	}
}

#[no_mangle]
pub extern "system" fn DllMain(_: HINSTANCE, call_reason: u32, _: LPVOID) -> BOOL {
	match call_reason {
		winnt::DLL_PROCESS_ATTACH => { std::thread::spawn(main); }
		_ => {}
	}

	TRUE
}
