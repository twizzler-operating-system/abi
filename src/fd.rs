pub use crate::bindings::descriptor as RawFd;

#[repr(u32)]
pub enum OpenError {
    Other = crate::bindings::open_error_OpenError_Other,
}

pub fn twz_rt_fd_open(_name: &str) -> Result<RawFd, OpenError> {
    todo!()
}

pub fn twz_rt_fd_close(_fd: RawFd) {
    todo!()
}
