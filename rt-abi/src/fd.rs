pub use crate::bindings::descriptor as RawFd;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u32)]
pub enum OpenError {
    Other = crate::bindings::open_error_OpenError_Other,
    LookupFail = crate::bindings::open_error_OpenError_LookupFail,
    PermissionDenied = crate::bindings::open_error_OpenError_PermissionDenied,
    InvalidArgument = crate::bindings::open_error_OpenError_InvalidArgument,
}

impl core::error::Error for OpenError {}

impl core::fmt::Display for OpenError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            OpenError::Other => write!(f, "unknown error"),
            OpenError::LookupFail => write!(f, "lookup fail"),
            OpenError::PermissionDenied => write!(f, "permission denied"),
            OpenError::InvalidArgument => write!(f, "invalid argument"),
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
