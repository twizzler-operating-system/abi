#![allow(unused_variables)]
use crate::fd::RawFd;


#[repr(u32)]
pub enum IoError {
    Other = crate::bindings::io_error_IoError_Other,
    WouldBlock = crate::bindings::io_error_IoError_WouldBlock,
}

bitflags::bitflags! {
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
    pub struct IoFlags : u32 {
        const NONBLOCKING = 1;
    }
}

pub enum SeekFrom {
    Start(usize),
    End(isize),
    Current(isize),
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

