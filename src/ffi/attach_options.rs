use libc::{c_char, c_int, c_void, c_long, gid_t, uid_t, uint64_t};

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