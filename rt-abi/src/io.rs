//! Runtime interface for IO-like operations.

#![allow(unused_variables)]
use crate::{fd::RawFd, Result};

bitflags::bitflags! {
    /// Possible flags for IO operations.
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
    pub struct IoFlags : crate::bindings::io_flags {
        /// This operation should have non-blocking semantics, regardless of fd status.
        const NONBLOCKING = crate::bindings::IO_NONBLOCKING;
    }
}

/// Possible seek start points and offset.
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

impl Into<Result<usize>> for crate::bindings::io_result {
    fn into(self) -> Result<usize> {
        let raw = TwzRawError::new(self.err);
        if raw.is_success() {
            Ok(self.val)
        } else {
            Err(raw.error())
        }
    }
}

impl From<Result<usize>> for crate::bindings::io_result {
    fn from(value: Result<usize>) -> Self {
        match value {
            Ok(v) => Self {
                val: v,
                err: crate::bindings::SUCCESS,
            },
            Err(e) => Self {
                val: 0,
                err: e.raw(),
            },
        }
    }
}

/// Read a file descriptor into a buffer, up to buf.len() bytes. On success, returns the number of
/// bytes actually read, which may be fewer than requested. If offset is None, use the file
/// descriptor's internal position. If the file descriptor refers to a non-seekable file, and offset
/// is Some, this function returns an error.
pub fn twz_rt_fd_pread(
    fd: RawFd,
    offset: Option<u64>,
    buf: &mut [u8],
    flags: IoFlags,
) -> Result<usize> {
    unsafe {
        crate::bindings::twz_rt_fd_pread(
            fd,
            optoff(offset),
            buf.as_mut_ptr().cast(),
            buf.len(),
            flags.bits(),
        )
        .into()
    }
}

/// Write bytes from a buffer into a file descriptor, up to buf.len() bytes. On success, returns the
/// number of bytes actually written, which may be fewer than requested. If offset is None, use the
/// file descriptor's internal position. If the file descriptor refers to a non-seekable file, and
/// offset is Some, this function returns an error.
pub fn twz_rt_fd_pwrite(
    fd: RawFd,
    offset: Option<u64>,
    buf: &[u8],
    flags: IoFlags,
) -> Result<usize> {
    unsafe {
        crate::bindings::twz_rt_fd_pwrite(
            fd,
            optoff(offset),
            buf.as_ptr().cast(),
            buf.len(),
            flags.bits(),
        )
        .into()
    }
}

/// Seek a file descriptor, changing the internal position.
pub fn twz_rt_fd_seek(fd: RawFd, seek: SeekFrom) -> Result<usize> {
    let (whence, off) = match seek {
        SeekFrom::Start(s) => (crate::bindings::WHENCE_START, s as i64),
        SeekFrom::End(s) => (crate::bindings::WHENCE_END, s),
        SeekFrom::Current(s) => (crate::bindings::WHENCE_CURRENT, s),
    };
    unsafe { crate::bindings::twz_rt_fd_seek(fd, whence, off).into() }
}

/// Type of an IO vec buffer and length.
pub type IoSlice = crate::bindings::io_vec;

/// Read a file descriptor into a multiple buffers. On success, returns the number of bytes actually
/// read, which may be fewer than requested. If offset is None, use the file descriptor's internal
/// position. If the file descriptor refers to a non-seekable file, and offset is Some, this
/// function returns an error.
pub fn twz_rt_fd_preadv(
    fd: RawFd,
    offset: Option<u64>,
    ios: &[IoSlice],
    flags: IoFlags,
) -> Result<usize> {
    unsafe {
        crate::bindings::twz_rt_fd_pwritev(
            fd,
            optoff(offset),
            ios.as_ptr(),
            ios.len(),
            flags.bits(),
        )
        .into()
    }
}

/// Write multiple buffers into a file descriptor. On success, returns the number of bytes actually
/// written, which may be fewer than requested. If offset is None, use the file descriptor's
/// internal position. If the file descriptor refers to a non-seekable file, and offset is Some,
/// this function returns an error.
pub fn twz_rt_fd_pwritev(
    fd: RawFd,
    offset: Option<u64>,
    ios: &[IoSlice],
    flags: IoFlags,
) -> Result<usize> {
    unsafe {
        crate::bindings::twz_rt_fd_pwritev(
            fd,
            optoff(offset),
            ios.as_ptr(),
            ios.len(),
            flags.bits(),
        )
        .into()
    }
}
