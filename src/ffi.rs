use libc::{c_char};

#[link(name = "lxc")]
extern {
  pub fn lxc_get_version() -> *const c_char;
}