use std::ffi::{CStr, CString, c_void};
use std::os::raw::c_char;

#[link(name = "readline")]
unsafe extern "C" {
	fn readline(prompt: *const c_char) -> *mut c_char;
	fn add_history(line: *const c_char);
	fn rl_initialize();
	fn free(ptr: *mut c_void); // appel direct à free sans libc
	fn clear_history(); // GNU Readline
}

pub fn clear_readline_history() {
	unsafe { clear_history(); }
}
pub struct Readline {
	history: Vec<String>,
}

impl Readline {
	pub fn new() -> Self {
		unsafe { rl_initialize(); }	// nécessaire sur macOS avec libedit
		Readline { history: Vec::new(), }
	}

	pub fn read_line(&mut self, prompt: &str, to_history: bool) -> Option<String> {
		let c_prompt = CString::new(prompt).ok()?;
		let ptr = unsafe { readline(c_prompt.as_ptr()) };

		if ptr.is_null() { return None; }

		let line = unsafe {
			let line = CStr::from_ptr(ptr).to_string_lossy().into_owned();
			free(ptr as *mut c_void); // libération explicite
			line
		};

		if !line.trim().is_empty() && to_history {
			if let Ok(c_line) = CString::new(line.clone()) {
				unsafe { add_history(c_line.as_ptr()); }
			}
			self.history.push(line.clone());
		}

		Some(line)
	}

	pub fn inject_history(&self) {
		for entry in &self.history {
			if let Ok(c_entry) = CString::new(entry.clone()) {
				unsafe {
					add_history(c_entry.as_ptr());
				}
			}
		}
	}
}
