use libc::{c_char};
use std::ffi::{CString, CStr};
use std::str;
use std::ptr;

/// Module containing helper functions, which will be used only internally.

/// Converts from `&str` to C pointer to c-string.
pub fn str_to_ptr(s: &str) -> CString {
    // Maybe better fail handling should be used
    CString::new(s).unwrap()
}

/// Converts from C pointer to c-string to `String`.
pub fn ptr_to_str(ptr: *const c_char) -> String {
  unsafe {
    let bytes = CStr::from_ptr(ptr).to_bytes();
    str::from_utf8(bytes).ok().expect("Invalid UTF8 string").to_string()
  }
}

pub fn vec_to_ptr(vec: Vec<&str>) -> Option<Vec<CString>> {
  if !vec.is_empty() {
    let mut tmp = vec.iter()
                     .map(|s| str_to_ptr(s))
                     .collect::<Vec<CString>>();
    Some(tmp)
  }
  else {
    None
  }
}