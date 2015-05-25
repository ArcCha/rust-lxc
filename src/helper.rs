use libc::{c_char};
use std::ffi::{CString, CStr};
use std::str;

/// Module containing helper functions, which will be used only internally.

/// Converts `&str` to `CString`.
pub fn str_to_cstring(s: &str) -> CString {
    // Maybe better fail handling should be used
    CString::new(s).unwrap()
}

/// Converts C-string pointer to `String`.
pub fn ptr_to_str(ptr: *const c_char) -> String {
  unsafe {
    let bytes = CStr::from_ptr(ptr).to_bytes();
    str::from_utf8(bytes).ok().expect("Invalid UTF8 string").to_string()
  }
}

/// Converts `&str` vector to `CString` vector
pub fn vec_str_to_cstring(vec: Vec<&str>) -> Vec<CString> {
    vec.iter()
       .map(|s| str_to_cstring(s))
       .collect::<Vec<CString>>()
}