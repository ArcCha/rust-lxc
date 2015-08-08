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

pub enum LxcCreateFlag {
  /// Leave stdin, stdout and stderr as is
  Verbose  = 0x00,
  /// Redirect stdin to /dev/zero and stdout and stderr to /dev/null
  Quiet    = 0x01,
  /// Number of LXC_CREATE_* flags
  Maxflags = 0x02,
}

pub enum LxcCloneFlag {
  /// Do not change the cloning behaviour
  Void          = 0x00,
  /// Do not edit the rootfs to change the hostname
  Keepname      = 0x01,
  /// Do not change the MAC address on network interfaces
  Keepmacaddr   = 0x02,
  /// Snapshot the original filesystem(s)
  Snapshot      = 0x04,
  /// Use the same bdev type
  Keepbdevtype  = 0x08,
  /// Snapshot only if bdev supports it, else copy
  MaybeSnapshot = 0x10,
  /// Number of LXC_CLONE_* flags
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
    let underlying = unsafe {
      let name_cstring = str_to_cstring(name);
      let name_ptr = name_cstring.as_ptr();
      let config_path_cstring;
      let config_path_ptr = match config_path_option {
        Some(config_path) =>  {  
                                config_path_cstring = str_to_cstring(config_path);
                                config_path_cstring.as_ptr() 
                              }
        None => ptr::null()
      };
      ffi::lxc_container_new(name_ptr, config_path_ptr)
    };
    LxcContainer::parse_creation_result(underlying)
  }

  fn parse_creation_result(underlying: *mut ffi::LxcContainer)
                                        -> Result<LxcContainer, &'static str> {
    if underlying.is_null() {
      Err("Cannot create LxcContainer")
    }
    else {
      Ok(LxcContainer { underlying: underlying })
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
      let config_path_cstring;
      let config_path_ptr = match config_path_option {
        Some(config_path) =>  {  
                                config_path_cstring = str_to_cstring(config_path);
                                config_path_cstring.as_ptr() 
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
      let argv_cstring;
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
  /// Returns `Ok` with config file name or `Err` in case of error.
  pub fn config_file_name(&self) -> Result<String, &'static str> {
    unsafe {
      let config_name_ptr = ((*self.underlying).config_file_name)(self.underlying);
      if config_name_ptr == ptr::null_mut() {
        Err("Couldn't get config file name")
      }
      else {
        Ok(ptr_to_str(config_name_ptr))
      }
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
  /// `config_save_path` - full path to file to save configuration in.
  ///
  /// # Return value
  /// Returns `true` on success, else `false`.
  pub fn save_config(&self, config_save_path: &str) -> bool {
    unsafe {
      let config_save_path_cstring = str_to_cstring(config_save_path);
      let config_save_path_ptr = config_save_path_cstring.as_ptr();
      ((*self.underlying).save_config)(self.underlying, config_save_path_ptr) != 0
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
      let bdevtype_cstring;
      let bdevtype_ptr = match bdevtype_option {
        Some(bdevtype) => {
                            bdevtype_cstring = str_to_cstring(bdevtype);
                            bdevtype_cstring.as_ptr()
                          }
        None => ptr::null()
      };
      let argv_cstring;
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

  /// Request the container reboot by sending it SIGINT.
  ///
  /// # Return value
  /// Returns `true` if reboot request successful, else `false`.
  pub fn reboot(&self) -> bool {
    unsafe {
      ((*self.underlying).reboot)(self.underlying) != 0
    }
  }

  /// Request the container shutdown by sending it SIGPWR.
  ///
  /// # Parameters
  /// `timeout` - seconds to wait before returning `false`: -1 to wait forever, 0 to avoid waiting.
  ///
  /// # Return value
  /// Returns `true` if the container was shutdown successfully, else `false`.
  pub fn shutdown(&self, timeout: i32) -> bool {
    unsafe {
      ((*self.underlying).shutdown)(self.underlying, timeout as libc::c_int) != 0
    }
  }

  /// Completely clear the containers in-memory configuration.
  pub fn clear_config(&self) {
    unsafe {
      ((*self.underlying).clear_config)(self.underlying)
    }
  }

  /// Clear a configuration item.
  ///
  /// # Parameters
  /// `key` - name of option to clear.
  ///
  /// # Return value
  /// Returns `true` on success, else `false`.
  ///
  /// ## Note
  /// Analog of `set_config_item()`.
  pub fn clear_config_item(&self, key: &str) -> bool {
    unsafe {
      let key_cstring = str_to_cstring(key);
      let key_ptr = key_cstring.as_ptr();
      ((*self.underlying).clear_config_item)(self.underlying, key_ptr) != 0
    }
  }

  /// Retrieve the value of a config item.
  ///
  /// # Parameters
  /// `key` - name of option to get.
  ///
  /// # Return value
  /// Returns `Ok` with the value of a config item or `Err` in case of error.
  pub fn get_config_item(&self, key: &str) -> Result<String, &'static str> {
    unsafe {
      let key_cstring = str_to_cstring(key);
      let key_ptr = key_cstring.as_ptr();
      let retv_len = ((*self.underlying).get_config_item)(self.underlying, key_ptr, ptr::null_mut(), 0);
      if retv_len == -1 {
        Err("Couldn't get config item value")
      }
      else {
        let mut retv = Vec::with_capacity(retv_len as usize);
        for _ in 0..retv_len {
          retv.push(' ' as libc::c_char);
        }
        ((*self.underlying).get_config_item)(self.underlying, key_ptr, retv.as_mut_ptr(), retv_len);
        retv.pop(); // pop null placed at the end
        Ok(String::from_utf8(retv.iter()
                              .map(|c| *c as u8)
                              .collect::<Vec<u8>>())
                              .ok().expect("Invalid UTF8 string"))
      }
    }
  }

  /// Retrieve the value of a config item from running container.
  ///
  /// # Parameters
  /// `key` - name of option to get.
  ///
  /// # Return value
  /// Returns `Ok` with the value of a config item or `Err` in case of error.
  pub fn get_running_config_item(&self, key: &str) -> Result<String, &'static str> {
    unsafe {
      let key_cstring = str_to_cstring(key);
      let key_ptr = key_cstring.as_ptr();
      let config_item_ptr = ((*self.underlying).get_running_config_item)(self.underlying, key_ptr);
      if config_item_ptr == ptr::null_mut() {
        Err("Couldn't get running config item value")
      }
      else {
        Ok(ptr_to_str(config_item_ptr))
      }
    }
  }

  /// Retrieve a list of config item keys given a key prefix.
  ///
  /// # Parameters
  /// `key_prefix` - name prefix of keys to get.
  ///
  /// # Return value
  /// Returns `Ok` with a vector of config item keys or `Err` in case of error.
  pub fn get_keys(&self, key_prefix: &str) -> Result<Vec<String>, &'static str> {
    unsafe {
      let key_prefix_cstring = str_to_cstring(key_prefix);
      let key_prefix_ptr = key_prefix_cstring.as_ptr();
      let retv_len = ((*self.underlying).get_keys)(self.underlying, key_prefix_ptr, ptr::null_mut(), 0);
      if retv_len == -1 {
        Err("Couldn't get list of config item keys")
      }
      else {
        let mut retv = Vec::with_capacity(retv_len as usize);
        for _ in 0..retv_len {
          retv.push(' ' as libc::c_char);
        }
        ((*self.underlying).get_keys)(self.underlying, key_prefix_ptr, retv.as_mut_ptr(), retv_len);
        retv.pop(); // pop null placed at the end
        let keys_newline_separated = String::from_utf8(retv.iter()
                                                           .map(|c| *c as u8)
                                                           .collect::<Vec<u8>>())
                                                           .ok().expect("Invalid UTF8 string");
        let keys_list : Vec<String> = keys_newline_separated.split('\n').map(|key| key.to_string()).collect();
        Ok(keys_list)
      }
    }
  }

  /// Obtain a list of network interfaces.
  ///
  /// # Return value
  /// Returns a vector of network interfaces.
  ///
  /// ## Note
  /// Due to limitation of the original C implementation this method returns empty vector 
  /// either when error occurred or no IP addresses are present. 
  pub fn get_interfaces(&self) -> Vec<String> {
    unsafe {
      let interfaces_ptr = ((*self.underlying).get_interfaces)(self.underlying);
      if interfaces_ptr == ptr::null_mut() {
        Vec::new()
      }
      else {
        let mut interfaces_list = Vec::new();
        let mut i = 0;
        loop {
          let interface_name_ptr = *interfaces_ptr.offset(i);
          if interface_name_ptr == ptr::null_mut() {
            break;
          }
          else {
            interfaces_list.push(ptr_to_str(interface_name_ptr));
          }
          i += 1;
        }
        interfaces_list
      }
    }
  }

  /// Determine the list of container IP addresses.
  ///
  /// # Parameters
  /// `interface_option` - option with network interface name to consider. 
  ///                      Pass `None` if you want to get IPs from all interfaces
  /// 
  /// `family_option` - option network family, for example "inet", "inet6". 
  ///                   Pass `None` if you want to get IPs from all families
  ///
  /// `scope` - IPv6 scope id (ignored if family is not "inet6").
  ///
  /// # Return value
  /// Returns a vector of container IP addresses.
  ///
  /// ## Note
  /// Due to limitation of the original C implementation this method returns empty vector 
  /// either when error occurred or no IP addresses are present. 
  pub fn get_ips(&self, interface_option: Option<&str>, 
                        family_option: Option<&str>, 
                        scope: i32) -> Vec<String> {
    unsafe {
      let interface_cstring;
      let interface_ptr = match interface_option {
        Some(interface) =>  {
                              interface_cstring = str_to_cstring(interface);
                              interface_cstring.as_ptr()
                            },
        None => ptr::null()
      };
      let family_cstring;
      let family_ptr = match family_option {
        Some(family) => {
                          family_cstring = str_to_cstring(family);
                          family_cstring.as_ptr()
                        },
        None => ptr::null()
      };
      let ips_ptr = ((*self.underlying).get_ips)(self.underlying, interface_ptr, family_ptr, scope);
      if ips_ptr == ptr::null_mut() {
        Vec::new()
      }
      else {
        let mut ips_list = Vec::new();
        let mut i = 0;
        loop {
          let ip_address_ptr = *ips_ptr.offset(i);
          if ip_address_ptr == ptr::null_mut() {
            break;
          }
          else {
            ips_list.push(ptr_to_str(ip_address_ptr));
          }
          i += 1;
        }
        ips_list
      }
    }
  }

  /// Retrieve the specified cgroup subsystem value for the container.
  ///
  /// # Parameters
  /// `subsys` - cgroup subsystem to retrieve.
  ///
  /// # Return value
  /// Returns `Ok` with `subsys` value or `Err` in case of error.
  pub fn get_cgroup_item(&self, subsys: &str) -> Result<String, &'static str> {
    unsafe {
      let subsys_cstring = str_to_cstring(subsys);
      let subsys_ptr = subsys_cstring.as_ptr();
      let retv_len = ((*self.underlying).get_cgroup_item)(self.underlying, subsys_ptr, ptr::null_mut(), 0);
      if retv_len == -1 {
        Err("Couldn't get specified cgroup subsystem value")
      }
      else {
        let mut retv = Vec::with_capacity(retv_len as usize);
        for _ in 0..retv_len {
          retv.push(' ' as libc::c_char);
        }
        ((*self.underlying).get_cgroup_item)(self.underlying, subsys_ptr, retv.as_mut_ptr(), retv_len);
        retv.pop(); // pop null placed at the end
        let subsys_value = String::from_utf8(retv.iter()
                                                 .map(|c| *c as u8)
                                                 .collect::<Vec<u8>>())
                                                 .ok().expect("Invalid UTF8 string");
        Ok(subsys_value)
      }
    }
  }

  /// Set the specified cgroup subsystem value for the container.
  ///
  /// # Parameters
  /// `subsys` - cgroup subsystem to consider.
  ///
  /// `value` - value to set for subsys.
  ///
  /// # Return value
  /// Returns `true` on success, else `false`.
  pub fn set_cgroup_item(&self, subsys: &str, value: &str) -> bool {
    unsafe {
      let subsys_cstring = str_to_cstring(subsys);
      let subsys_ptr = subsys_cstring.as_ptr();
      let value_cstring = str_to_cstring(value);
      let value_ptr = value_cstring.as_ptr();
      ((*self.underlying).set_cgroup_item)(self.underlying, subsys_ptr, value_ptr) != 0
    }
  }

  /// Determine full path to the containers configuration file.
  ///  
  /// ## Note
  /// Each container can have a custom configuration path. However by default it will be set to 
  /// either the LXCPATH configure variable, or the lxcpath value in the LXC_GLOBAL_CONF configuration file 
  /// (i.e. /etc/lxc/lxc.conf). The value for a specific container can be changed using set_config_path(). 
  /// There is no other way to specify this in general at the moment.
  /// 
  /// # Returns
  /// Returns full path to configuration file.
  pub fn get_config_path(&self) -> String {
    unsafe {
      let config_path_ptr = ((*self.underlying).get_config_path)(self.underlying);
      ptr_to_str(config_path_ptr)
    }
  }

  /// Set the full path to the containers configuration file.
  /// 
  /// # Parameters
  /// `config_path_option` - `Option` with full path to config file. If you don't want to use any, pass `None`.
  /// 
  /// # Returns
  /// Returns `true` on success, else `false`.
  pub fn set_config_path(&self, config_path_option: Option<&str>) -> bool {
    unsafe {
      let config_path_cstring;
      let config_path_ptr = match config_path_option {
        Some(config_path) =>  {  
                                config_path_cstring = str_to_cstring(config_path);
                                config_path_cstring.as_ptr() 
                              }
        None => ptr::null()
      };
      ((*self.underlying).set_config_path)(self.underlying, config_path_ptr) != 0
    }
  }

  
  /// Copy a stopped container.
  ///
  /// # Parameters
  /// `newname` - `Option` with new name for the container. Pass None if you
  ///  want the same name to be used. Then, a new lxcpath MUST be specified.
  ///
  /// `lxcpath` - `Option` with lxcpath in which to create the new container.
  /// Pass None if you want the original container's lxcpath to be used.
  ///
  /// `flags` - additional `LXC_CLONE_*` flags to change the cloning behaviour:
  ///
  ///  - [`Keepname`](enum.LxcCloneFlag.html)
  ///  - [`Keepmacaddr`](enum.LxcCloneFlag.html)
  ///  - [`Snapshot`](enum.LxcCloneFlag.html)
  ///
  /// `bdevtype` - optionally force the cloned bdevtype to a specified plugin.
  ///  By default (pass None) the original is used (subject to snapshot
  ///  requirements). //TODO not sure about None//
  ///
  /// `bdevdata` - information about how to create the new storage
  ///  (i.e. fstype and fsdata).
  ///
  /// `newsize` - in case of a block device backing store, an
  ///  optional size. If `0`, the original backing store's size will
  ///  be used if possible. Note this only applies to the rootfs. For
  ///  any other filesystems, the original size will be duplicated.
  ///
  /// `hookargs` - additional arguments to pass to the clone hook script.
  ///
  /// # Returns
  ///
  /// Newly-allocated copy of container, or `Err` on error.
  ///
  /// # Note
  ///
  /// If `devtype` was not specified, and `flags` contains
  /// [`Snapshot`](enum.LxcCloneFlag.html) then use the native `bdevtype`
  /// if possible, else use an overlayfs.
  ///
  pub fn clone(&self,
              newname: Option<&str>,
              lxcpath: Option<&str>,
              flags: LxcCloneFlag,
              bdevtype: Option<&str>,
              bdevdata: Option<&str>,
              newsize: u64,
              argv_option: Option<Vec<&str>>)
                                        -> Result<LxcContainer, &'static str> {
    unsafe {
      let newname_cstring;
      let newname_ptr = match newname {
        Some(name) => {
                        newname_cstring = str_to_cstring(name);
                        newname_cstring.as_ptr()
                      }
        None => ptr::null()
      };
      let lxcpath_cstring;
      let lxcpath_ptr = match lxcpath {
        Some(path) => {
                        lxcpath_cstring = str_to_cstring(path);
                        lxcpath_cstring.as_ptr()
                      }
        None => ptr::null()
      };
      let bdevtype_cstring;
      let bdevtype_ptr = match bdevtype {
        Some(bdev) => {
                        bdevtype_cstring = str_to_cstring(bdev);
                        bdevtype_cstring.as_ptr()
                      }
        None => ptr::null()
      };
      let bdevdata_cstring;
      let bdevdata_ptr = match bdevdata {
        Some(data) => {
                        bdevdata_cstring = str_to_cstring(data);
                        bdevdata_cstring.as_ptr()
                      }
        None => ptr::null()
      };
      let argv_cstring;
      let argv_ptr = match argv_option {
        Some(argv) => {
                        argv_cstring = vec_str_to_cstring(argv);
                        argv_cstring.iter()
                                    .map(|s| s.as_ptr() as *mut libc::c_char)
                                    .collect::<Vec<*mut libc::c_char>>()
                                    .as_mut_ptr()
                      }
        None => ptr::null_mut()
      };
      let cloned = ((*self.underlying).clone)(self.underlying, newname_ptr,
                                              lxcpath_ptr, flags as i32,
                                              bdevtype_ptr, bdevdata_ptr,
                                              newsize, argv_ptr);
      LxcContainer::parse_creation_result(cloned)
    }
  }
}

/// Specifications for how to create a new backing store.
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