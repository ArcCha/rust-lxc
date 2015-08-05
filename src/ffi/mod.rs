use libc::{c_char, c_void, c_int, pid_t, uint64_t};

pub mod lxclock;
use self::lxclock::LxcLock; // self::lxclock::* do not work - why?
pub mod attach_options;
use self::attach_options::*;

pub const LXC_CLONE_KEEPNAME        : c_int = 0x01;
pub const LXC_CLONE_KEEPMACADDR     : c_int = 0x02;
pub const LXC_CLONE_SNAPSHOT        : c_int = 0x04;
pub const LXC_CLONE_KEEPBDEVTYPE    : c_int = 0x08;
pub const LXC_CLONE_MAYBE_SNAPSHOT  : c_int = 0x10;
pub const LXC_CLONE_MAXFLAGS        : c_int = 0x20;
pub const LXC_CREATE_QUIET          : c_int = 0x01;
pub const LXC_CREATE_MAXFLAGS       : c_int = 0x02;

#[link(name = "lxc")]
extern {
  pub fn lxc_container_new(name: *const c_char,
                           configpath: *const c_char)
                          -> *mut LxcContainer;
  pub fn lxc_container_get(c: *mut LxcContainer) -> c_int;
  pub fn lxc_container_put(c: *mut LxcContainer) -> c_int;
  pub fn lxc_get_wait_states(states: *const*const c_char) -> c_int;
  pub fn lxc_get_global_config_item(key: *const c_char) -> *const c_char;
  pub fn lxc_get_version() -> *const c_char;
  pub fn list_defined_containers(lxcpath: *const c_char,
                                 names: *mut*mut*mut c_char,
                                 cret: *mut*mut*mut LxcContainer) -> c_int;
  pub fn list_active_containers(lxcpath: *const c_char,
                                names: *mut*mut*mut c_char,
                                cret: *mut*mut*mut LxcContainer) -> c_int;
  pub fn list_all_containers(lxcpath: *const c_char,
                             names: *mut*mut*mut c_char,
                             cret: *mut*mut*mut LxcContainer) -> c_int;
  pub fn lxc_log_close();
}

#[repr(C)]
pub struct LxcSnapshot {
    pub name: *mut c_char,
    pub comment_pathname: *mut c_char,
    pub timestamp: *mut c_char,
    pub lxcpath: *mut c_char,
    pub free: extern fn(s: *mut LxcSnapshot),
}

#[repr(C)]
pub struct LxcConf;

/// For documentation see: https://github.com/lxc/lxc/blob/master/src/lxc/lxccontainer.h
#[repr(C)]
pub struct LxcContainer {
  name: *mut c_char,
  configfile: *mut c_char,
  pidfile: *mut c_char,
  slock: *mut LxcLock,
  privlock: *mut LxcLock,
  numthreads: c_int,
  lxc_conf: *mut LxcConf,

  pub error_string: *mut c_char,
  pub error_num: c_int,
  pub daemonize: c_char,
  pub config_path: *mut c_char,

  pub is_defined: extern fn(c: *mut LxcContainer) -> c_char,
  pub state: extern fn(c: *mut LxcContainer) -> *const c_char,
  pub is_running: extern fn(c: *mut LxcContainer) -> c_char,
  pub freeze: extern fn(c: *mut LxcContainer) -> c_char,
  pub unfreeze: extern fn(c: *mut LxcContainer) -> c_char,
  pub init_pid: extern fn(c: *mut LxcContainer) -> pid_t,
  pub load_config: extern fn(c: *mut LxcContainer, 
                             alt_file: *const c_char) -> c_char,
  pub start: extern fn(c: *mut LxcContainer, 
                       useinit: c_int, 
                       argv: *const*const c_char) -> c_char,
  pub startl: extern fn(c: *mut LxcContainer, 
                        useinit: c_int,...) -> c_char,
  pub stop: extern fn(c: *mut LxcContainer) -> c_char,
  pub want_daemonize: extern fn(c: *mut LxcContainer, 
                                state: c_char) -> c_char,
  pub want_close_all_fds: extern fn(c: *mut LxcContainer, 
                                    state: c_char) -> c_char,
  pub config_file_name: extern fn(c: *mut LxcContainer) -> *mut c_char,
  pub wait: extern fn(c: *mut LxcContainer, 
                      state: *const c_char, 
                      timeout: c_int) -> c_char,
  pub set_config_item: extern fn(c: *mut LxcContainer, 
                                 key: *const c_char, 
                                 value: *const c_char) -> c_char,
  pub destroy: extern fn(c: *mut LxcContainer) -> c_char,
  pub destroy_with_snapshots: extern fn(c: *mut LxcContainer) -> c_char,
  pub save_config: extern fn(c: *mut LxcContainer, 
                             alt_file: *const c_char) -> c_char,
  pub create: extern fn(c: *mut LxcContainer, 
                        t: *const c_char, 
                        bdevtype: *const c_char, 
                        specs: *mut BDevSpecs, 
                        flags: c_int, 
                        argv: *const*const c_char) -> c_char,
  pub createl: extern fn(c: *mut LxcContainer, t: *const c_char, 
                         bdevtype: *const c_char, 
                         specs: *mut BDevSpecs, 
                         flags: c_int,...) -> c_char,
  pub rename: extern fn(c: *mut LxcContainer, 
                        newname: *const c_char) -> c_char,
  pub reboot: extern fn(c: *mut LxcContainer) -> c_char,
  pub shutdown: extern fn(c: *mut LxcContainer, 
                          timeout: c_int) -> c_char,
  pub clear_config: extern fn(c: *mut LxcContainer),
  pub clear_config_item: extern fn(c: *mut LxcContainer, 
                                   key: *const c_char) -> c_char,
  pub get_config_item: extern fn(c: *mut LxcContainer, 
                                 key: *const c_char, 
                                 retv: *mut c_char, 
                                 inlen: c_int) -> c_int,
  pub get_running_config_item: extern fn(c: *mut LxcContainer, 
                                         key: *const c_char) -> *mut c_char,
  pub get_keys: extern fn(c: *mut LxcContainer, 
                          key: *const c_char,
                          retv: *mut c_char, 
                          inlen: c_int) -> c_int,
  pub get_interfaces: extern fn(c: *mut LxcContainer) -> *mut*mut c_char,
  pub get_ips: extern fn(c: *mut LxcContainer, 
                         interface: *const c_char, 
                         family: *const c_char, 
                         scope: c_int) -> *mut*mut c_char,
  pub get_cgroup_item: extern fn(c: *mut LxcContainer, 
                                 subsys: *const c_char, 
                                 retv: *mut c_char, 
                                 inlen: c_int) -> c_int,
  pub set_cgroup_item: extern fn(c: *mut LxcContainer, 
                                 subsys: *const c_char, 
                                 value: *const c_char) -> c_char,
  pub get_config_path: extern fn(c: *mut LxcContainer) -> *const c_char,
  pub set_config_path: extern fn(c: *mut LxcContainer, 
                                 path: *const c_char) -> c_char,
  pub clone: extern fn(c: *mut LxcContainer, 
                       newname: *const c_char, 
                       lxcpath: *const c_char, 
                       flags: c_int, 
                       bdevtype: *const c_char, 
                       bdevdata: *const c_char, 
                       newsize: uint64_t, 
                       hookargs: *mut*mut c_char) -> *mut LxcContainer,
  pub console_getfd: extern fn(c: *mut LxcContainer, 
                               ttynum: *mut c_int, 
                               masterfd: *mut c_int) -> c_int,
  pub console: extern fn(c: *mut LxcContainer, 
                         ttynum: c_int, 
                         stdinfd: c_int, 
                         stdoutfd: c_int, 
                         stderrfd: c_int, 
                         escape: c_int) -> c_int,
  pub attach: extern fn(c: *mut LxcContainer, 
                        exec_function: extern fn(payload: *mut c_void) -> c_int, 
                        exec_payload: *mut c_void, 
                        options: *mut LxcAttachOptions, 
                        attached_process: *mut pid_t) -> c_int,
  pub attach_run_wait: extern fn(c: *mut LxcContainer, 
                                 options: *mut LxcAttachOptions, 
                                 program: *const c_char, 
                                 argv: *const*const c_char) -> c_int,
  pub attach_run_waitl: extern fn(c: *mut LxcContainer, 
                                  options: *mut LxcAttachOptions, 
                                  program: *const c_char,
                                  arg: *const c_char,...) -> c_int,
  pub snapshot: extern fn(c: *mut LxcContainer, 
                          commentfile: *const c_char) -> c_int,
  pub snapshot_list: extern fn(c: *mut LxcContainer, 
                               snapshots: *mut*mut LxcSnapshot) -> c_int,
  pub snapshot_restore: extern fn(c: *mut LxcContainer, 
                                  snapname: *const c_char, 
                                  newname: *const c_char) -> c_char,
  pub snapshot_destroy: extern fn(c: *mut LxcContainer, 
                                 snapname: *const c_char) -> c_char,
  pub snapshot_destroy_all: extern fn(c: *mut LxcContainer) -> c_char,
  pub may_control: extern fn(c: *mut LxcContainer) -> c_char,
  pub add_device_node: extern fn(c: *mut LxcContainer, 
                                 src_path: *const c_char, 
                                 dest_path: *const c_char) -> c_char,
  pub remove_device_node: extern fn(c: *mut LxcContainer, 
                                    src_path: *const c_char, 
                                    dest_path: *const c_char) -> c_char,
  pub attach_interface: extern fn(c: *mut LxcContainer, 
                                  dev: *const c_char, 
                                  dst_dev: *const c_char) -> c_char,
  pub detach_interface: extern fn(c: *mut LxcContainer, 
                                  dev: *const c_char,
                                  dst_dev: *const c_char) -> c_char,
  pub checkpoint: extern fn(c: *mut LxcContainer, 
                            directory: *mut c_char, 
                            stop: c_char, 
                            verbose: c_char) -> c_char,
  pub restore: extern fn(c: *mut LxcContainer, 
                         directory: *mut c_char, 
                         verbose: c_char) -> c_char
}