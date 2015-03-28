extern crate libc;

use std::ffi::CStr;
use std::str;
mod ffi;

pub fn version() -> String {
  unsafe {
    let ptr = ffi::lxc_get_version();
    let bytes = CStr::from_ptr(ptr).to_bytes();
    str::from_utf8(bytes).ok().expect("Invalid UTF8 string").to_string()
  }
}