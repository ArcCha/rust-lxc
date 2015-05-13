use super::LxcContainer;
use libc::{c_int, c_char, c_short};

pub const LXC_LOCK_ANON_SEM : i32 = 0x00000001; // originally 1
pub const LXC_LOCK_FLOCK    : i32 = 0x00000002; // originally 2

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

#[link(name = "lxc")] // Need to tell the compiler to what we want these functions to be linked
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