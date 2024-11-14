#![allow(unused_variables)]
use crate::fd::RawFd;

#[repr(u32)]
pub enum IoError {
    Other = crate::bindings::io_error_IoError_Other,
    WouldBlock = crate::bindings::io_error_IoError_WouldBlock,
    SeekError = crate::bindings::io_error_IoError_SeekError,
    InvalidDesc = crate::bindings::io_error_IoError_InvalidDesc,
}

bitflags::bitflags! {
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
    pub struct IoFlags : crate::bindings::io_flags {
        const NONBLOCKING = crate::bindings::IO_NONBLOCKING;
    }
}

impl TryFrom<crate::bindings::io_error> for IoError {
    type Error = ();
    fn try_from(value: crate::bindings::io_error) -> Result<IoError, ()> {
        Ok(match value { 
            crate::bindings::io_error_IoError_Other => IoError::Other, 
            crate::bindings::io_error_IoError_WouldBlock => IoError::WouldBlock, 
            crate::bindings::io_error_IoError_SeekError => IoError::SeekError, 
            crate::bindings::io_error_IoError_InvalidDesc => IoError::InvalidDesc,
            n if n != crate::bindings::io_error_IoError_Success => IoError::Other,
            _ => return Err(()),
        })
    }
}


pub enum SeekFrom {
    Start(u64),
    End(i64),
    Current(i64),
}

fn optoff(off: Option<u64>) -> crate::bindings::optional_offset {
    match off {
        Some(o) => o.try_into().unwrap_or(crate::bindings::FD_POS),
        None => crate::bindings::FD_POS,
    }
}

impl Into<Result<usize, IoError>> for crate::bindings::io_result {
    fn into(self) -> Result<usize, IoError> {
        if let Ok(e) = self.error.try_into() {
            return Err(e);
        }
        Ok(self.value)
    }
}

impl From<Result<usize, IoError>> for crate::bindings::io_result {
    fn from(value: Result<usize, IoError>) -> Self {
        match value {
            Ok(v) => Self {
                value: v,
                error: crate::bindings::io_error_IoError_Success,
            },
            Err(e) => Self {
                value: 0,
                error: e as u32,
            },
        }
    }
}

pub fn twz_rt_fd_pread(fd: RawFd, offset: Option<u64>, buf: &mut [u8], flags: IoFlags) -> Result<usize, IoError> {
    unsafe { crate::bindings::twz_rt_fd_pread(fd, optoff(offset), buf.as_mut_ptr().cast(), buf.len(), flags.bits()).into() }
}

pub fn twz_rt_fd_pwrite(fd: RawFd, offset: Option<u64>, buf: &[u8], flags: IoFlags) -> Result<usize, IoError> {
    unsafe { crate::bindings::twz_rt_fd_pwrite(fd, optoff(offset), buf.as_ptr().cast(), buf.len(), flags.bits()).into() }
}

pub fn twz_rt_fd_seek(fd: RawFd, seek: SeekFrom) -> Result<usize, IoError> {
    let (whence, off) = match seek {
        SeekFrom::Start(s) => (crate::bindings::WHENCE_START, s as i64),
        SeekFrom::End(s) => (crate::bindings::WHENCE_END, s),
        SeekFrom::Current(s) => (crate::bindings::WHENCE_CURRENT, s),
    };
    unsafe {
        crate::bindings::twz_rt_fd_seek(fd, whence, off).into()
    }
}

pub type IoSlice = crate::bindings::io_vec;

pub fn twz_rt_fd_preadv(fd: RawFd, offset: Option<u64>, ios: &[IoSlice], flags: IoFlags) -> Result<usize, IoError> {
    unsafe { crate::bindings::twz_rt_fd_pwritev(fd, optoff(offset), ios.as_ptr(), ios.len(), flags.bits()).into() }
}

pub fn twz_rt_fd_pwritev(fd: RawFd, offset: Option<u64>, ios: &[IoSlice], flags: IoFlags) -> Result<usize, IoError> {
    unsafe { crate::bindings::twz_rt_fd_pwritev(fd, optoff(offset), ios.as_ptr(), ios.len(), flags.bits()).into() }
}
