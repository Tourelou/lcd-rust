// wrapper_sqlite3.rs

use std::ffi::{CString, CStr, c_void};
use std::os::raw::{c_char, c_int};
use std::ptr;

const SQLITE_OK: c_int = 0;

#[link(name = "sqlite3")]
unsafe extern "C" {
	fn sqlite3_open(filename: *const c_char, pp_db: *mut *mut c_void) -> c_int;
	fn sqlite3_close(db: *mut c_void) -> c_int;
	fn sqlite3_exec(
		db: *mut c_void,
		sql: *const c_char,
		callback: Option<extern "C" fn(*mut c_void, c_int, *mut *mut c_char, *mut *mut c_char) -> c_int>,
		arg: *mut c_void,
		errmsg: *mut *mut c_char,
	) -> c_int;
	fn sqlite3_free(ptr: *mut c_void);
}

pub struct Connection {
	handle: Option<*mut c_void>, // ← évite double close
}

impl Connection {
	pub fn open(path: &str) -> Result<Self, String> {
		let c_path = CString::new(path).map_err(|e| format!("sqlite3 error: Invalid path: {}", e))?;
		let mut db: *mut c_void = ptr::null_mut();

		let rc = unsafe { sqlite3_open(c_path.as_ptr(), &mut db) };
		if rc != SQLITE_OK || db.is_null() {
			Err("sqlite3 error: Failed to open database".to_string())
		}
		else { Ok(Connection { handle: Some(db) }) }
	}

	pub fn exec(&self, sql: &str) -> Result<(), String> {
		let c_sql = CString::new(sql).map_err(|e| format!("sqlite3 error: Invalid SQL: {}", e))?;
		let mut errmsg: *mut c_char = ptr::null_mut();

		let rc = unsafe {
			sqlite3_exec(
				self.handle.unwrap(),
				c_sql.as_ptr(),
				None,
				ptr::null_mut(),
				&mut errmsg,
			)
		};

		if rc != SQLITE_OK {
			let msg = if !errmsg.is_null() {
				let err = unsafe { CStr::from_ptr(errmsg).to_string_lossy().into_owned() };
				unsafe { sqlite3_free(errmsg as *mut c_void) };
				err
			}
			else { "Unknown error".to_string() };
			Err(format!("sqlite3 error: {}", msg))
		}
		else { Ok(()) }
	}

	pub fn query(&self, sql: &str) -> Result<Vec<Vec<(String, String)>>, String> {
		let c_sql = CString::new(sql).map_err(|e| format!("sqlite3 error: Invalid SQL: {}", e))?;
		let mut errmsg: *mut c_char = ptr::null_mut();
		let mut results: Vec<Vec<(String, String)>> = Vec::new();
		let results_ptr = &mut results as *mut _ as *mut c_void;

		extern "C" fn c_callback(
			arg: *mut c_void,
			argc: c_int,
			argv: *mut *mut c_char,
			col_names: *mut *mut c_char,
		) -> c_int {
			unsafe {
				if argv.is_null() || col_names.is_null() || arg.is_null() {
					return 1;
				}

				let results = &mut *(arg as *mut Vec<Vec<(String, String)>>);
				let values = std::slice::from_raw_parts(argv, argc as usize);
				let columns = std::slice::from_raw_parts(col_names, argc as usize);

				let mut row = Vec::new();
				for i in 0..(argc as usize) {
					let col = match CStr::from_ptr(columns[i]).to_str() {
						Ok(s) => s.to_owned(),
						Err(_) => "<invalid utf8>".to_string(),
					};
					let val = if values[i].is_null() {
						"".to_string()
					} else {
						match CStr::from_ptr(values[i]).to_str() {
							Ok(s) => s.to_owned(),
							Err(_) => "<invalid utf8>".to_string(),
						}
					};
					row.push((col, val));
				}

				results.push(row);
			}
			0
		}

		let rc = unsafe {
			sqlite3_exec(
				self.handle.unwrap(),
				c_sql.as_ptr(),
				Some(c_callback),
				results_ptr,
				&mut errmsg,
			)
		};

		if rc != SQLITE_OK {
			let msg = if !errmsg.is_null() {
				let err = unsafe { CStr::from_ptr(errmsg).to_string_lossy().into_owned() };
				unsafe { sqlite3_free(errmsg as *mut c_void) };
				err
			} else {
				"Unknown error".to_string()
			};
			Err(format!("sqlite3 error: {}", msg))
		} else {
			Ok(results)
		}
	}
}

impl Drop for Connection {
	fn drop(&mut self) {
		if let Some(db) = self.handle.take() {
			unsafe {
				let rc = sqlite3_close(db);
				if rc != SQLITE_OK {
					eprintln!("sqlite3 error: Error closing database (code: {})", rc);
				}
			}
		}
	}
}
