use libc::{c_char, c_void, c_int};

// Not sure if it should be here
// pub const LXC_CLONE_KEEPNAME        : u8 = 1 << 0;
// pub const LXC_CLONE_KEEPMACADDR     : u8 = 1 << 1;
// pub const LXC_CLONE_SNAPSHOT        : u8 = 1 << 2;
// pub const LXC_CLONE_KEEPBDEVTYPE    : u8 = 1 << 3;
// pub const LXC_CLONE_MAYBE_SNAPSHOT  : u8 = 1 << 4;
// pub const LXC_CLONE_MAXFLAGS        : u8 = 1 << 5;
// pub const LXC_CREATE_QUIET          : u8 = 1 << 0;
// pub const LXC_CREATE_MAXFLAGS       : u8 = 1 << 1;

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
pub struct LxcLock;

#[repr(C)]
pub struct LxcContainer {
  name: *mut c_char,
  configfile: *mut c_char,
  pidfile: *mut c_char,
  slock: *mut LxcLock,
  privlock: *mut LxcLock,
  numthreads: c_int,

}