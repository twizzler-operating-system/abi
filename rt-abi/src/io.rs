//! Runtime interface for IO-like operations.

#![allow(unused_variables)]
use crate::{
    error::RawTwzError,
    fd::{RawFd, SocketAddress},
    Result,
};

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
        let raw = RawTwzError::new(self.err);
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
                err: RawTwzError::success().raw(),
            },
            Err(e) => Self {
                val: 0,
                err: e.raw(),
            },
        }
    }
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct IoCtx(crate::bindings::io_ctx);

impl Default for IoCtx {
    fn default() -> Self {
        Self::new(None, IoFlags::empty(), None)
    }
}

impl IoCtx {
    pub fn new(pos: Option<u64>, flags: IoFlags, timeout: Option<core::time::Duration>) -> Self {
        Self(crate::bindings::io_ctx {
            offset: optoff(pos),
            flags: flags.bits(),
            timeout: timeout.into(),
        })
    }

    pub fn offset(mut self, offset: Option<u64>) -> Self {
        self.0.offset = optoff(offset);
        self
    }

    pub fn flags(mut self, flags: IoFlags) -> Self {
        self.0.flags = flags.bits();
        self
    }

    pub fn timeout(mut self, timeout: Option<core::time::Duration>) -> Self {
        self.0.timeout = timeout.into();
        self
    }
}

/// Read a file descriptor into a buffer, up to buf.len() bytes. On success, returns the number of
/// bytes actually read, which may be fewer than requested.
pub fn twz_rt_fd_pread(fd: RawFd, buf: &mut [u8], ctx: &mut IoCtx) -> Result<usize> {
    unsafe {
        crate::bindings::twz_rt_fd_pread(fd, buf.as_mut_ptr().cast(), buf.len(), &mut ctx.0).into()
    }
}

/// Write bytes from a buffer into a file descriptor, up to buf.len() bytes. On success, returns the
/// number of bytes actually written, which may be fewer than requested.
pub fn twz_rt_fd_pwrite(fd: RawFd, buf: &[u8], ctx: &mut IoCtx) -> Result<usize> {
    unsafe {
        crate::bindings::twz_rt_fd_pwrite(fd, buf.as_ptr().cast(), buf.len(), &mut ctx.0).into()
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

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Endpoint(crate::bindings::endpoint);

impl Endpoint {
    fn new_socket(sock: super::fd::SocketAddress) -> Self {
        Self(crate::bindings::endpoint {
            kind: crate::bindings::endpoint_kind_Endpoint_Socket,
            addr: crate::bindings::endpoint_addrs {
                socket_addr: sock.0,
            },
        })
    }
}

#[repr(u32)]
pub enum EndpointKind {
    Unspecified = crate::bindings::endpoint_kind_Endpoint_Unspecified,
    Socket = crate::bindings::endpoint_kind_Endpoint_Socket,
}

impl From<crate::bindings::endpoint> for Endpoint {
    fn from(value: crate::bindings::endpoint) -> Self {
        Self(value)
    }
}

impl From<SocketAddress> for Endpoint {
    fn from(value: SocketAddress) -> Self {
        Self::new_socket(value)
    }
}

/// Read a file descriptor into a buffer, up to buf.len() bytes. On success, returns the number of
/// bytes actually read, which may be fewer than requested.
pub fn twz_rt_fd_pread_from(
    fd: RawFd,
    buf: &mut [u8],
    ctx: &mut IoCtx,
) -> Result<(usize, Endpoint)> {
    let mut endpoint = core::mem::MaybeUninit::uninit();
    unsafe {
        let len: Result<_> = crate::bindings::twz_rt_fd_pread_from(
            fd,
            buf.as_mut_ptr().cast(),
            buf.len(),
            &mut ctx.0,
            endpoint.as_mut_ptr(),
        )
        .into();
        let len = len?;
        Ok((len, endpoint.assume_init().into()))
    }
}

/// Write bytes from a buffer into a file descriptor, up to buf.len() bytes. On success, returns the
/// number of bytes actually written, which may be fewer than requested.
pub fn twz_rt_fd_pwrite_to(
    fd: RawFd,
    buf: &[u8],
    ctx: &mut IoCtx,
    mut ep: Endpoint,
) -> Result<usize> {
    unsafe {
        crate::bindings::twz_rt_fd_pwrite_to(
            fd,
            buf.as_ptr().cast(),
            buf.len(),
            &mut ctx.0,
            &mut ep.0,
        )
        .into()
    }
}

/// Type of an IO vec buffer and length.
pub type IoSlice = crate::bindings::io_vec;

/// Read a file descriptor into a multiple buffers. On success, returns the number of bytes actually
/// read, which may be fewer than requested. If offset is None, use the file descriptor's internal
/// position. If the file descriptor refers to a non-seekable file, and offset is Some, this
/// function returns an error.
pub fn twz_rt_fd_preadv(fd: RawFd, ios: &[IoSlice], ctx: &mut IoCtx) -> Result<usize> {
    unsafe { crate::bindings::twz_rt_fd_pwritev(fd, ios.as_ptr(), ios.len(), &mut ctx.0).into() }
}

/// Write multiple buffers into a file descriptor. On success, returns the number of bytes actually
/// written, which may be fewer than requested. If offset is None, use the file descriptor's
/// internal position. If the file descriptor refers to a non-seekable file, and offset is Some,
/// this function returns an error.
pub fn twz_rt_fd_pwritev(fd: RawFd, ios: &[IoSlice], ctx: &mut IoCtx) -> Result<usize> {
    unsafe { crate::bindings::twz_rt_fd_pwritev(fd, ios.as_ptr(), ios.len(), &mut ctx.0).into() }
}
