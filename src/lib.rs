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
  pub fn new(name: &str, config_path: &str) -> Option<LxcContainer> {
    let tmp = LxcContainer {
      container: unsafe {
        if config_path == "" {
          ffi::lxc_container_new(str_to_ptr(name),
                                 ptr::null::<libc::c_char>())
        }
        else {
          ffi::lxc_container_new(str_to_ptr(name),
                                 str_to_ptr(config_path))
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

  pub fn is_running(&self) -> bool {
    unsafe {
      ((*self.container).is_running)(self.container) != 0
    }
  }

  pub fn freeze(&self) -> bool {
    unsafe {
      ((*self.container).freeze)(self.container) != 0
    }
  }

  pub fn unfreeze(&self) -> bool {
    unsafe {
      ((*self.container).unfreeze)(self.container) != 0
    }
  }

  pub fn init_pid(&self) -> i32 {
    unsafe {
      ((*self.container).init_pid)(self.container)
    }
  }

  pub fn load_config(&self, config_path: &str) -> bool {
    unsafe {
      let mut config_path_ptr = ptr::null::<libc::c_char>();
      if config_path != "" {
        config_path_ptr = str_to_ptr(config_path);
      }
      ((*self.container).load_config)(self.container, config_path_ptr) != 0
    }
  }

  pub fn start(&self, use_init: i32, argv: Vec<&str>) -> bool {
    unsafe {
      let mut argv_ptr = ptr::null::<*const libc::c_char>();
      if !argv.is_empty() {
        argv_ptr = argv.iter()
                   .map(|s| str_to_ptr(s))
                   .collect::<Vec<*const libc::c_char>>()
                   .as_ptr() as *const*const libc::c_char
      }                               
      ((*self.container).start)(self.container, use_init, argv_ptr) != 0
    }
  }

  pub fn stop(&self) -> bool {
    unsafe {
      ((*self.container).stop)(self.container) != 0
    }
  }

  pub fn want_daemonize(&self, state: bool) -> bool {
    unsafe {
      ((*self.container).want_daemonize)(self.container, state as libc::c_char) != 0
    }
  }

  pub fn want_close_all_fds(&self, state: bool) -> bool {
    unsafe {
      ((*self.container).want_close_all_fds)(self.container, state as libc::c_char) != 0
    }
  }

  pub fn config_file_name(&self) -> String {
    unsafe {
      ptr_to_str(((*self.container).config_file_name)(self.container))
    }
  }

  pub fn wait(&self, state: &str, timeout: i32) -> bool {
    unsafe {
      ((*self.container).wait)(self.container, str_to_ptr(state), timeout as libc::c_int) != 0
    }
  }

  pub fn set_config_item(&self, key: &str, value: &str) -> bool {
    unsafe {
      ((*self.container).set_config_item)(self.container, str_to_ptr(key), str_to_ptr(value)) != 0
    }
  }

  pub fn destroy(&self) -> bool {
    unsafe {
      ((*self.container).destroy)(self.container) != 0
    }
  }

  pub fn destroy_with_snapshots(&self) -> bool {
    unsafe {
      ((*self.container).destroy_with_snapshots)(self.container) != 0
    }
  }

  pub fn save_config(&self, alt_file: &str) -> bool {
    unsafe {
      ((*self.container).save_config)(self.container, str_to_ptr(alt_file)) != 0
    }
  }

  pub fn rename(&self, new_name: &str) -> bool {
    unsafe {
      ((*self.container).rename)(self.container, str_to_ptr(new_name)) != 0
    }
  }
}