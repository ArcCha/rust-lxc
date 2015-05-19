extern crate libc;

mod ffi;
mod helper;
use helper::*;
use std::ptr;

/// Returns liblxc version.
///
/// # Example
/// ```
/// let lxc_version = liblxc::version();
/// ```
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

pub enum LxcCreateFlag {
  Quiet    = 0x01,
  Maxflags = 0x02,
}

pub enum LxcCloneFlag {
  Keepname      = 0x01,
  Keepmacaddr   = 0x02,
  Snapshot      = 0x04,
  Keepbdevtype  = 0x08,
  MaybeSnapshot = 0x10,
  Maxflags      = 0x20,
}

pub enum LxcAttachFlag {
  MoveToCgroup     = 0x00000001,
  DropCapabilities = 0x00000002,
  SetPersonality   = 0x00000004,
  LsmExec          = 0x00000008,
  RemountProcSys   = 0x00010000,
  LsmNow           = 0x00020000,
  Default          = 0x0000FFFF,
  // Lsm = (LsmExec | LsmNow),
  Lsm              = 0x00020008,
}

/// Struct representing lxc container.
pub struct LxcContainer {
    container: *mut ffi::LxcContainer
}

impl LxcContainer {

  /// Creates new lxc container object - it does not create lxc container in the host system.
  ///
  /// # Parameters
  /// `name` - name to use for the container.
  /// 
  /// `config_path` - full path to optional config file. If you don't want to use any, pass empty string.
  ///
  /// # Return value
  /// Returns `Ok(LxcContainer)` if the creation was sucessful, else `Err(&'static str)` with error description.
  ///
  /// # Examples
  /// ```
  /// let c = liblxc::LxcContainer::new("example", "");
  /// # assert!(c.is_ok())
  /// ```
  pub fn new(name: &str, config_path: &str) -> Result<LxcContainer, &'static str> {
    let tmp = LxcContainer {
      container: unsafe {
        ffi::lxc_container_new(str_to_ptr(name),
                               str_to_ptr(config_path))
      }
    };

    if tmp.container.is_null() {
      Err("cannot create LxcContainer")
    }
    else {
      Ok(tmp)
    }
  }

  /// Determine if /var/lib/lxc/$name/config exists.
  ///
  /// # Return value
  /// Returns `true` if container is defined, else `false`.
  pub fn is_defined(&self) -> bool {
    unsafe {
      ((*self.container).is_defined)(self.container) != 0
    }
  }

  /// Determine state of container.
  ///
  /// # Return value
  /// Returns upper-case string representing state of container.
  pub fn state(&self) -> String { // maybe define an enum with possible states instead returning String?
    unsafe {
      ptr_to_str(((*self.container).state)(self.container))
    }
  }

  /// Determine if container is running.
  ///
  /// # Return value
  /// Returns `true` when container is running, else `false`.
  pub fn is_running(&self) -> bool {
    unsafe {
      ((*self.container).is_running)(self.container) != 0
    }
  }

  /// Freeze running container.
  ///
  /// # Return value
  /// Returns `true` on success, else `false`.
  pub fn freeze(&self) -> bool {
    unsafe {
      ((*self.container).freeze)(self.container) != 0
    }
  }

  /// Thaw a frozen container.
  ///
  /// # Return value
  /// Returns `true` on success, else `false`.
  pub fn unfreeze(&self) -> bool {
    unsafe {
      ((*self.container).unfreeze)(self.container) != 0
    }
  }

  /// Determine process ID of the containers init process.
  ///
  /// # Return value
  /// Returns pid of init process as seen from outside the container.
  pub fn init_pid(&self) -> i32 {
    unsafe {
      ((*self.container).init_pid)(self.container)
    }
  }

  /// Load the specified configuration for the container.
  ///
  /// # Parameters
  /// `config_path` - full path to alternate configuration file, or empty string to use the default configuration file. 
  ///
  /// # Return value
  /// Returns `true` on success, else `false`.
  pub fn load_config(&self, config_path: &str) -> bool {
    unsafe {
      let mut config_path_ptr = ptr::null::<libc::c_char>();
      if config_path != "" {
        config_path_ptr = str_to_ptr(config_path);
      }
      ((*self.container).load_config)(self.container, config_path_ptr) != 0
    }
  }

  /// Start the container.
  ///
  /// # Parameters
  /// `use_init` - use lxcinit rather than /sbin/init.
  ///
  /// `argv` - vector of arguments to pass to init.
  ///
  /// # Return value
  /// Returns `true` on success, else `false`.
  pub fn start(&self, use_init: i32, argv: Vec<&str>) -> bool {
    unsafe {
      let argv_ptr = vec_to_ptr(argv);                               
      ((*self.container).start)(self.container, use_init, argv_ptr) != 0
    }
  }

  /// Stop the container.
  ///
  /// # Return value
  /// Returns `true` on success, else `false`.
  pub fn stop(&self) -> bool {
    unsafe {
      ((*self.container).stop)(self.container) != 0
    }
  }

  /// Determine if the container wants to run disconnected from the terminal.
  /// 
  /// # Parameters
  /// `state` - value for the `daemonize` bit.
  /// 
  /// # Return value
  /// Returns `true` if container wants to be daemonized, else `false`.
  pub fn want_daemonize(&self, state: bool) -> bool {
    unsafe {
      ((*self.container).want_daemonize)(self.container, state as libc::c_char) != 0
    }
  }

  /// Determine whether container wishes all file descriptors to be closed on startup.
  ///
  /// # Parameters
  /// `state` - value for the `close_all_fds` bit
  ///
  /// # Return value
  /// Returns `true` if container wants all file descriptors closed, else `false`.
  pub fn want_close_all_fds(&self, state: bool) -> bool {
    unsafe {
      ((*self.container).want_close_all_fds)(self.container, state as libc::c_char) != 0
    }
  }

  /// Return current config file name.
  ///
  /// # Return value
  /// Returns config file name.
  pub fn config_file_name(&self) -> String {
    unsafe {
      // TODO returns NULL on error
      ptr_to_str(((*self.container).config_file_name)(self.container))
    }
  }

  /// Wait for container to reach a particular state.
  ///
  /// # Parameters
  /// `state` - state to wait for.
  ///
  /// `timeout` - timeout in seconds.
  ///
  /// # Return value
  /// Returns `true` if state reached within timeout, else `false`.
  pub fn wait(&self, state: &str, timeout: i32) -> bool {
    unsafe {
      ((*self.container).wait)(self.container, str_to_ptr(state), timeout as libc::c_int) != 0
    }
  }

  /// Set a key/value configuration option.
  ///
  /// # Parameters
  /// `key` - name of option to set.
  ///
  /// `value` - value of `name` to set.
  /// 
  /// # Return value
  /// Returns `true` on success, else `false`.
  pub fn set_config_item(&self, key: &str, value: &str) -> bool {
    unsafe {
      ((*self.container).set_config_item)(self.container, str_to_ptr(key), str_to_ptr(value)) != 0
    }
  }

  /// Delete the container.
  ///
  /// ## Note
  /// Container must be stopped and have no dependent snapshots.
  /// 
  /// # Return value
  /// Returns `true` on success, else `false`.
  pub fn destroy(&self) -> bool {
    unsafe {
      ((*self.container).destroy)(self.container) != 0
    }
  }

  /// Delete the container and all its snapshots.
  ///
  /// ## Note
  /// Container must be stopped.
  /// 
  /// # Return value
  /// Returns `true` on success, else `false`.
  pub fn destroy_with_snapshots(&self) -> bool {
    unsafe {
      ((*self.container).destroy_with_snapshots)(self.container) != 0
    }
  }

  /// Save configuaration to a file.
  ///
  /// # Parameters
  /// `alt_file` - full path to file to save configuration in.
  ///
  /// # Return value
  /// Returns `true` on success, else `false`.
  pub fn save_config(&self, alt_file: &str) -> bool {
    unsafe {
      ((*self.container).save_config)(self.container, str_to_ptr(alt_file)) != 0
    }
  }

  /// Rename a container.
  ///
  /// # Parameters
  /// `new_name` - new name to be used for the container.
  ///
  /// # Return value
  /// Returns `true` on success, else `false`.
  pub fn rename(&self, new_name: &str) -> bool {
    unsafe {
      ((*self.container).rename)(self.container, str_to_ptr(new_name)) != 0
    }
  }

  pub fn create(&self, template: &str, bdevtype: &str, bdev_specs: BDevSpecs,
                flags: LxcCreateFlag, argv: Vec<&str>) -> bool {
    unsafe {
      let argv_ptr = vec_to_ptr(argv);
      ((*self.container).create)(self.container, str_to_ptr(template),
                                str_to_ptr(bdevtype),
                                bdev_specs.underlying, flags as libc::c_int,
                                argv_ptr) != 0
    }
  }
}

pub struct BDevSpecs {
    underlying: *mut ffi::attach_options::BDevSpecs
}

impl BDevSpecs {
  pub fn new() -> BDevSpecs {
    BDevSpecs {
      underlying: ptr::null_mut::<ffi::attach_options::BDevSpecs>()
    }
  }  
}