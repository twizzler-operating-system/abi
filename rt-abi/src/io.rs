#![allow(unused_variables)]
use crate::fd::RawFd;

#[repr(u32)]
pub enum IoError {
    Other = crate::bindings::io_error_IoError_Other,
    WouldBlock = crate::bindings::io_error_IoError_WouldBlock,
}

bitflags::bitflags! {
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
    pub struct IoFlags : crate::bindings::io_flags {
        const NONBLOCKING = crate::bindings::IO_NONBLOCKING;
    }
}

pub enum SeekFrom {
    Start(u64),
    End(i64),
    Current(i64),
}

pub fn twz_rt_fd_read(fd: RawFd, buf: &mut [u8], flags: IoFlags) -> Result<usize, IoError> {
    todo!()
}

pub fn twz_rt_fd_write(fd: RawFd, buf: &[u8], flags: IoFlags) -> Result<usize, IoError> {
    todo!()
}

pub fn twz_rt_fd_seek(fd: RawFd, seek: SeekFrom) -> Result<usize, IoError> {
    todo!()
}

pub type IoSlice = crate::bindings::io_vec;

pub fn twz_rt_fd_preadv(fd: RawFd, offset: Option<u64>, ios: &[IoSlice], flags: IoFlags) -> Result<usize, IoError> {
    todo!()
}

pub fn twz_rt_fd_pwritev(fd: RawFd, offset: Option<u64>, ios: &[IoSlice], flags: IoFlags) -> Result<usize, IoError> {
    todo!()
}
