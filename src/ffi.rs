use libc::{c_char, c_void, c_int, c_short, c_long, pid_t, gid_t, uid_t, uint64_t};

// Not sure if it should be here
pub const LXC_CLONE_KEEPNAME        : u8 = 1 << 0;
pub const LXC_CLONE_KEEPMACADDR     : u8 = 1 << 1;
pub const LXC_CLONE_SNAPSHOT        : u8 = 1 << 2;
pub const LXC_CLONE_KEEPBDEVTYPE    : u8 = 1 << 3;
pub const LXC_CLONE_MAYBE_SNAPSHOT  : u8 = 1 << 4;
pub const LXC_CLONE_MAXFLAGS        : u8 = 1 << 5;
pub const LXC_CREATE_QUIET          : u8 = 1 << 0;
pub const LXC_CREATE_MAXFLAGS       : u8 = 1 << 1;

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

// BEGIN ---------- lxclock.h -------------
pub const LXC_LOCK_ANON_SEM : i32 = 1;
pub const LXC_LOCK_FLOCK    : i32 = 2;
#[repr(C)]
struct FileDescriptor {
  fd: c_int,
  fname: *mut c_char
}
#[repr(C)]

pub struct LxcLock {
  type_: c_short,
  u: FileDescriptor
}
//struct lxc_lock {
//    short type; 
//    union {
//        sem_t *sem;  // sizeof(sem_t*) = 8
//        struct { 
//            int   fd; 
//            char *fname; 
//        } f; // sizeof(f) = 16
//    } u; 
//};
//
extern {
  fn lxc_newlock(lxcpath: *const c_char, name: *const c_char) -> *mut LxcLock;
  fn lxclock(lock: *mut LxcLock, timeout: c_int) -> c_int;
  fn lxcunlock(lock: *mut LxcLock) -> c_int;
  fn lxc_putlock(lock: *mut LxcLock);
  fn process_lock();
  fn process_unlock();
  fn container_mem_lock(c: *mut LxcContainer) -> c_int;
  fn container_mem_unlock(c: *mut LxcContainer);
  fn container_disk_lock(c: *mut LxcContainer) -> c_int;
  fn container_disk_unlock(c: *mut LxcContainer);
}
// END ---------- lxclock.h ------------- 

#[repr(C)]
pub struct LxcConf;

// BEGIN ---------- attach_options.h ------------- 
#[repr(C)]
struct ZfsRoot {
  zfsroot: *mut c_char
}

#[repr(C)]
struct Lvm {
  vg: *mut c_char,
  lv: *mut c_char,
  thinpool: *mut c_char
}

#[repr(C)]
pub struct BDevSpecs {
  pub fstype: *mut c_char,
  fssize: uint64_t,
  zfs: ZfsRoot,
  lvm: Lvm,
  dir: *mut c_char
}

#[repr(C)]
pub enum LxcAttachEnvPolicy {
  LXC_ATTACH_KEEP_ENV,
  LXC_ATTACH_CLEAR_ENV
}

#[repr(C)]
// enabled by default
pub const LXC_ATTACH_MOVE_TO_CGROUP     : u32 = 0x00000001;
pub const LXC_ATTACH_DROP_CAPABILITIES  : u32 = 0x00000002; 
pub const LXC_ATTACH_SET_PERSONALITY    : u32 = 0x00000004; 
pub const LXC_ATTACH_LSM_EXEC           : u32 = 0x00000008; 
// the following are off by default
pub const LXC_ATTACH_REMOUNT_PROC_SYS   : u32 = 0x00010000; 
pub const LXC_ATTACH_LSM_NOW            : u32 = 0x00020000;
// enable some options
pub const LXC_ATTACH_DEFAULT            : u32 = 0x0000FFFF;
pub const LXC_ATTACH_LSM                : u32 = (LXC_ATTACH_LSM_EXEC | LXC_ATTACH_LSM_NOW);

#[repr(C)]
pub struct LxcAttachOptions {
  attach_flags: c_int,
  namespaces: c_int,
  personality: c_long,
  initial_cwd: *mut c_char,
  uid: uid_t,
  gid: gid_t,
  env_policy: LxcAttachEnvPolicy,
  extra_env_vars: *mut*mut c_char,
  extra_keep_env: *mut*mut c_char,
  stdin_fd: c_int,
  stdout_fd: c_int, 
  stderr_fd: c_int, 
}
#[repr(C)]
pub struct LxcAttachCommand {
  program: *mut c_char,
  argv: *mut*mut c_char
}

#[link(name = "lxc")]
extern {
  pub fn lxc_attach_run_command(payload: *mut c_void) -> c_int;
  pub fn lxc_attach_run_shell(payload: *mut c_void) -> c_int;
}
// END ---------- attach_options.h ------------- 

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