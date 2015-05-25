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
  Verbose  = 0x00,
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
  Lsm              = (LxcAttachFlag::LsmExec as isize |
                      LxcAttachFlag::LsmNow as isize),
}

/// Struct representing lxc container.
pub struct LxcContainer {
    underlying: *mut ffi::LxcContainer
}

impl LxcContainer {

  /// Creates new lxc container object - it does not create lxc container in the host system.
  ///
  /// # Parameters
  /// `name` - name to use for the container.
  /// 
  /// `config_path_option` - `Option` with full path to config file. If you don't want to use any, pass `None`.
  ///
  /// # Return value
  /// Returns `Ok(LxcContainer)` if the creation was sucessful, else `Err(&'static str)` with error description.
  ///
  /// # Examples
  /// ```
  /// let c = liblxc::LxcContainer::new("example", "");
  /// # assert!(c.is_ok())
  /// ```
  pub fn new(name: &str, config_path_option: Option<&str>) -> Result<LxcContainer, &'static str> {
    let container = LxcContainer {
      underlying: unsafe {
        let name_cstring = str_to_cstring(name);
        let name_ptr = name_cstring.as_ptr();
        let mut config_path_cstring;
        let config_path_ptr = match config_path_option {
          Some(config_path) =>  {  
                                  config_path_cstring = str_to_cstring(config_path);
                                  str_to_cstring(config_path).as_ptr() 
                                }
          None => ptr::null()
        };
        ffi::lxc_container_new(name_ptr, config_path_ptr)
      }
    };

    if container.underlying.is_null() {
      Err("Cannot create LxcContainer")
    }
    else {
      Ok(container)
    }
  }

  /// Determine if /var/lib/lxc/$name/config exists.
  ///
  /// # Return value
  /// Returns `true` if container is defined, else `false`.
  pub fn is_defined(&self) -> bool {
    unsafe {
      ((*self.underlying).is_defined)(self.underlying) != 0
    }
  }

  /// Determine state of container.
  ///
  /// # Return value
  /// Returns upper-case string representing state of container.
  pub fn state(&self) -> String { // maybe define an enum with possible states instead returning String?
    unsafe {
      ptr_to_str(((*self.underlying).state)(self.underlying))
    }
  }

  /// Determine if container is running.
  ///
  /// # Return value
  /// Returns `true` when container is running, else `false`.
  pub fn is_running(&self) -> bool {
    unsafe {
      ((*self.underlying).is_running)(self.underlying) != 0
    }
  }

  /// Freeze running container.
  ///
  /// # Return value
  /// Returns `true` on success, else `false`.
  pub fn freeze(&self) -> bool {
    unsafe {
      ((*self.underlying).freeze)(self.underlying) != 0
    }
  }

  /// Thaw a frozen container.
  ///
  /// # Return value
  /// Returns `true` on success, else `false`.
  pub fn unfreeze(&self) -> bool {
    unsafe {
      ((*self.underlying).unfreeze)(self.underlying) != 0
    }
  }

  /// Determine process ID of the containers init process.
  ///
  /// # Return value
  /// Returns pid of init process as seen from outside the container.
  pub fn init_pid(&self) -> i32 {
    unsafe {
      ((*self.underlying).init_pid)(self.underlying)
    }
  }

  /// Load the specified configuration for the container.
  ///
  /// # Parameters
  /// `config_path_option` - `Option` with full path to alternate configuration file, or `None` to use the default. 
  ///
  /// # Return value
  /// Returns `true` on success, else `false`.
  pub fn load_config(&self, config_path_option: Option<&str>) -> bool {
    unsafe {
      let mut config_path_cstring;
      let config_path_ptr = match config_path_option {
        Some(config_path) =>  {  
                                config_path_cstring = str_to_cstring(config_path);
                                str_to_cstring(config_path).as_ptr() 
                              }
        None => ptr::null()
      };
      ((*self.underlying).load_config)(self.underlying, config_path_ptr) != 0
    }
  }

  /// Start the container.
  ///
  /// # Parameters
  /// `use_init` - use lxcinit rather than /sbin/init.
  ///
  /// `argv_option` - `Option` with vector of arguments to pass to init. If no arguments are required, pass `None`.
  ///
  /// # Return value
  /// Returns `true` on success, else `false`.
  pub fn start(&self, use_init: i32, argv_option: Option<Vec<&str>>) -> bool {
    unsafe {
      let mut argv_cstring;
      let argv_ptr = match argv_option {
        Some(argv) => {
                        argv_cstring = vec_str_to_cstring(argv);
                        argv_cstring.iter()
                                    .map(|s| s.as_ptr())
                                    .collect::<Vec<*const libc::c_char>>()
                                    .as_ptr()
                      }
        None => ptr::null()
      };
      ((*self.underlying).start)(self.underlying, use_init as libc::c_int, argv_ptr) != 0
    }
  }

  /// Stop the container.
  ///
  /// # Return value
  /// Returns `true` on success, else `false`.
  pub fn stop(&self) -> bool {
    unsafe {
      ((*self.underlying).stop)(self.underlying) != 0
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
      ((*self.underlying).want_daemonize)(self.underlying, state as libc::c_char) != 0
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
      ((*self.underlying).want_close_all_fds)(self.underlying, state as libc::c_char) != 0
    }
  }

  /// Return current config file name.
  ///
  /// # Return value
  /// Returns config file name.
  pub fn config_file_name(&self) -> String {
    unsafe {
      // TODO care: returns NULL on error
      ptr_to_str(((*self.underlying).config_file_name)(self.underlying))
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
      let state_cstring = str_to_cstring(state);
      let state_ptr = state_cstring.as_ptr();
      ((*self.underlying).wait)(self.underlying, state_ptr, timeout as libc::c_int) != 0
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
      let key_cstring = str_to_cstring(key);
      let key_ptr = key_cstring.as_ptr();
      let value_cstring = str_to_cstring(value);
      let value_ptr = value_cstring.as_ptr();
      ((*self.underlying).set_config_item)(self.underlying, key_ptr, value_ptr) != 0
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
      ((*self.underlying).destroy)(self.underlying) != 0
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
      ((*self.underlying).destroy_with_snapshots)(self.underlying) != 0
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
      let alt_file_cstring = str_to_cstring(alt_file);
      let alt_file_ptr = alt_file_cstring.as_ptr();
      ((*self.underlying).save_config)(self.underlying, alt_file_ptr) != 0
    }
  }

  /// Create a container.
  ///
  /// # Parameters
  /// `template` - template to execute to instantiate the root filesystem and adjust the configuration.
  ///
  /// `bdevtype_option` - `Option` with backing store type to use. If `None`, dir will be used.
  ///
  /// `specs` - additional parameters for the backing store (for example LVM volume group to use).
  ///
  /// `flags` - `LxcCreateFlag` options
  ///
  /// `argv_option` - `Option` with vector of arguments to pass to the template. If no arguments are required, pass `None`.
  ///
  /// # Return value
  /// Returns `true` on success, else `false`.
  pub fn create(&self, template: &str, 
                       bdevtype_option: Option<&str>, 
                       bdev_specs: BDevSpecs,
                       flags: LxcCreateFlag, 
                       argv_option: Option<Vec<&str>>) -> bool {
    unsafe {
      let template_cstring = str_to_cstring(template);
      let template_ptr = template_cstring.as_ptr();
      let mut bdevtype_cstring;
      let bdevtype_ptr = match bdevtype_option {
        Some(bdevtype) => {
                            bdevtype_cstring = str_to_cstring(bdevtype);
                            bdevtype_cstring.as_ptr()
                          }
        None => ptr::null()
      };
      let mut argv_cstring;
      let argv_ptr = match argv_option {
        Some(argv) => {
                        argv_cstring = vec_str_to_cstring(argv);
                        argv_cstring.iter()
                                    .map(|s| s.as_ptr())
                                    .collect::<Vec<*const libc::c_char>>()
                                    .as_ptr()
                      }
        None => ptr::null()
      };
      ((*self.underlying).create)(self.underlying, 
                                  template_ptr,
                                  bdevtype_ptr,
                                  bdev_specs.underlying,
                                  flags as libc::c_int,
                                  argv_ptr) != 0
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
      let new_name_cstring = str_to_cstring(new_name);
      let new_name_ptr = new_name_cstring.as_ptr();
      ((*self.underlying).rename)(self.underlying, new_name_ptr) != 0
    }
  }
}

pub struct BDevSpecs {
    underlying: *mut ffi::attach_options::BDevSpecs
}

impl BDevSpecs {
  pub fn new() -> BDevSpecs {
    BDevSpecs {
      underlying: ptr::null_mut()
    }
  }  
}