extern crate libc;

mod ffi;
mod helper;
use helper::*;
use std::ptr;

pub fn version() -> String {
  unsafe {
    let ptr = ffi::lxc_get_version();
    ptr_to_str(ptr)
  }
}

// impl ffi::LxcSnapshot {
//   fn free(&mut self) {
//     unsafe {
//       (self.free)(self)
//     }
//   }
// }

pub struct LxcContainer {
    container: *mut ffi::LxcContainer
}

impl LxcContainer {
  pub fn new(name: &str, configpath: &str) -> Option<LxcContainer> {
    let tmp = LxcContainer {
      container: unsafe {
        if configpath == "" {
          ffi::lxc_container_new(str_to_ptr(name),
                                 ptr::null::<libc::c_char>())
        }
        else {
          ffi::lxc_container_new(str_to_ptr(name),
                                 str_to_ptr(configpath))
        }
      }
    };

    if tmp.container.is_null() {
      None
    }
    else {
      Some(tmp)
    }
  }

  pub fn is_defined(&self) -> bool {
    unsafe {
      ((*self.container).is_defined)(self.container) != 0
    }
  }

  pub fn state(&self) -> String { // maybe define an enum with possible states instead returning String?
    unsafe {
      ptr_to_str(((*self.container).state)(self.container))
    }
  }

  pub fn rename(&self, new_name: &str) -> bool {
    unsafe {
      ((*self.container).rename)(self.container, str_to_ptr(new_name)) != 0
    }
  }
}