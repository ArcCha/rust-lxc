use libc::{c_char};
use std::ffi::{CString, CStr};
use std::str;

/// Module containing helper functions, which will be used only internally.

/// Converts from `&str` to C pointer to c-string.
pub fn str_to_ptr(s: &str) -> *const c_char {
  CString::new(s).unwrap().as_ptr()
}

/// Converts from C pointer to c-string to `String`.
pub fn ptr_to_str(ptr: *const c_char) -> String {
  unsafe {
    let bytes = CStr::from_ptr(ptr).to_bytes();
    str::from_utf8(bytes).ok().expect("Invalid UTF8 string").to_string()
  }
}