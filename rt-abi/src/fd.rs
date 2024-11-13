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

impl TryFrom<crate::bindings::open_error> for OpenError {
    type Error = ();
    fn try_from(value: crate::bindings::open_error) -> Result<OpenError, ()> {
        Ok(match value { 
            crate::bindings::open_error_OpenError_Other => OpenError::Other, 
            crate::bindings::open_error_OpenError_LookupFail => OpenError::LookupFail, 
            crate::bindings::open_error_OpenError_PermissionDenied => OpenError::PermissionDenied, 
            crate::bindings::open_error_OpenError_InvalidArgument => OpenError::InvalidArgument,
            n if n != crate::bindings::open_error_OpenError_Success => OpenError::Other,
            _ => return Err(()),
        })
    }
}

impl From<Result<RawFd, OpenError>> for crate::bindings::open_result {
    fn from(value: Result<RawFd, OpenError>) -> Self {
        match value {
            Ok(fd) => Self {
                error: crate::bindings::open_error_OpenError_Success,
                fd,
            },
            Err(e) => Self {
                error: e as crate::bindings::open_error,
                fd: 0,
            }
        }
    }
}

bitflags::bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct FdFlags : crate::bindings::fd_flags {
    const IS_TERMINAL = crate::bindings::FD_IS_TERMINAL;
}
}

#[derive(Copy, Clone, Debug)]
pub struct FdInfo {
    pub flags: FdFlags,
}

impl From<crate::bindings::fd_info> for FdInfo {
    fn from(value: crate::bindings::fd_info) -> Self {
        Self {
            flags: FdFlags::from_bits_truncate(value.flags)
        }
    }
}

pub fn twz_rt_fd_get_info(fd: RawFd) -> Option<FdInfo> {
    let mut info = core::mem::MaybeUninit::uninit();
    unsafe {
        if crate::bindings::twz_rt_fd_get_info(fd, info.as_mut_ptr()) {
            return Some(info.assume_init().into());
        }
    }
    None
}

pub fn twz_rt_fd_copen(name: &core::ffi::CStr) -> Result<RawFd, OpenError> {
    let info = crate::bindings::open_info {
        name: name.as_ptr().cast(),
        len: name.count_bytes(),
    };
    unsafe {
        let result = crate::bindings::twz_rt_fd_open(info);
        if let Ok(err) = result.error.try_into() {
            return Err(err);
        }
        Ok(result.fd)
    }
}

pub fn twz_rt_fd_open(name: &str) -> Result<RawFd, OpenError> {
    let info = crate::bindings::open_info {
        name: name.as_ptr().cast(),
        len: name.len(),
    };
    unsafe {
        let result = crate::bindings::twz_rt_fd_open(info);
        if let Ok(err) = result.error.try_into() {
            return Err(err);
        }
        Ok(result.fd)
    }
}

pub fn twz_rt_fd_dup(fd: RawFd) -> Result<RawFd, OpenError> {
    let mut new_fd = core::mem::MaybeUninit::<RawFd>::uninit();
    unsafe {
        if crate::bindings::twz_rt_fd_cmd(fd, crate::bindings::FD_CMD_DUP, core::ptr::null_mut(), new_fd.as_mut_ptr().cast()) == crate::bindings::FD_CMD_SUCCESS {
            return Ok(new_fd.assume_init());
        }
    }
    Err(OpenError::Other)
}

pub fn twz_rt_fd_close(fd: RawFd) {
    unsafe {
        crate::bindings::twz_rt_fd_close(fd)
    }
}
