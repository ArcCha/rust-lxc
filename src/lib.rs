extern crate libc;

mod ffi;
mod helper;
use helper::*;

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
        ffi::lxc_container_new(str_to_ptr(name),
                               str_to_ptr(configpath))
      }
    };

    if tmp.container.is_null() {
      None
    }
    else {
      Some(tmp)
    }
  }

  pub fn rename(&self, new_name: &str) -> bool {
    unsafe {
      ((*self.container).rename)(self.container, str_to_ptr(new_name)) != 0
    }
  }
}