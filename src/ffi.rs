use libc::{c_char};

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
  pub fn lxc_get_version() -> *const c_char;
}