pub use crate::bindings::descriptor as RawFd;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u32)]
pub enum OpenError {
    Other = crate::bindings::open_error_OpenError_Other,
}

impl core::error::Error for OpenError {}

impl core::fmt::Display for OpenError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            OpenError::Other => write!(f, "unknown error"),
        }
    }
}

pub fn twz_rt_fd_copen(_name: &core::ffi::CStr) -> Result<RawFd, OpenError> {
    todo!()
}

pub fn twz_rt_fd_open(_name: &str) -> Result<RawFd, OpenError> {
    todo!()
}

pub fn twz_rt_fd_dup(_fd: RawFd) -> Result<RawFd, OpenError> {
    todo!()
}

pub fn twz_rt_fd_close(_fd: RawFd) {
    todo!()
}
