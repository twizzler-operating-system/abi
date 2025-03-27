//! Runtime interface for file descriptors.

use core::time::Duration;

pub use crate::bindings::descriptor as RawFd;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u32)]
/// Possible Open error states.
pub enum OpenError {
    /// Unclassified errror.
    Other = crate::bindings::open_error_OpenError_Other,
    /// Lookup failed.
    LookupFail = crate::bindings::open_error_OpenError_LookupFail,
    /// Permission denied.
    PermissionDenied = crate::bindings::open_error_OpenError_PermissionDenied,
    /// An argument was invalid.
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

impl From<OpenError> for crate::bindings::open_error {
    fn from(value: OpenError) -> crate::bindings::open_error {
        match value {
            OpenError::Other => crate::bindings::open_error_OpenError_Other,
            OpenError::LookupFail => crate::bindings::open_error_OpenError_LookupFail,
            OpenError::PermissionDenied => crate::bindings::open_error_OpenError_PermissionDenied,
            OpenError::InvalidArgument => crate::bindings::open_error_OpenError_InvalidArgument,
        }
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
            },
        }
    }
}

bitflags::bitflags! {
    /// Flags for file descriptors.
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct FdFlags : crate::bindings::fd_flags {
    /// The file descriptor refers to a terminal.
    const IS_TERMINAL = crate::bindings::FD_IS_TERMINAL;
}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default)]
#[repr(u32)]
/// Possible Fd Kinds
pub enum FdKind {
    #[default]
    Regular = crate::bindings::fd_kind_FdKind_Regular,
    Directory = crate::bindings::fd_kind_FdKind_Directory,
    SymLink = crate::bindings::fd_kind_FdKind_SymLink,
    Other = u32::MAX,
}

impl From<u32> for FdKind {
    fn from(value: u32) -> Self {
        match value {
            crate::bindings::fd_kind_FdKind_Regular => Self::Regular,
            crate::bindings::fd_kind_FdKind_Directory => Self::Directory,
            crate::bindings::fd_kind_FdKind_SymLink => Self::SymLink,
            _ => Self::Other,
        }
    }
}

impl Into<u32> for FdKind {
    fn into(self) -> u32 {
        match self {
            Self::Regular => crate::bindings::fd_kind_FdKind_Regular,
            Self::Directory => crate::bindings::fd_kind_FdKind_Directory,
            Self::SymLink => crate::bindings::fd_kind_FdKind_SymLink,
            Self::Other => u32::MAX,
        }
    }
}

/// Information about an open file descriptor.
#[derive(Copy, Clone, Debug, Default)]
pub struct FdInfo {
    /// Flags for this descriptor
    pub flags: FdFlags,
    /// Length of underlying object
    pub size: u64,
    /// Kind of file
    pub kind: FdKind,
    /// Object ID
    pub id: twizzler_types::ObjID,
    /// Created time
    pub created: Duration,
    /// Accessed time
    pub accessed: Duration,
    /// Modified time
    pub modified: Duration,
    /// Unix mode
    pub unix_mode: u32,
}

impl From<crate::bindings::fd_info> for FdInfo {
    fn from(value: crate::bindings::fd_info) -> Self {
        Self {
            flags: FdFlags::from_bits_truncate(value.flags),
            size: value.len,
            kind: FdKind::from(value.kind),
            id: value.id,
            created: value.created.into(),
            accessed: value.accessed.into(),
            modified: value.modified.into(),
            unix_mode: value.unix_mode,
        }
    }
}

impl From<FdInfo> for crate::bindings::fd_info {
    fn from(value: FdInfo) -> Self {
        Self {
            flags: value.flags.bits(),
            len: value.size,
            kind: value.kind.into(),
            id: value.id,
            created: value.created.into(),
            accessed: value.accessed.into(),
            modified: value.modified.into(),
            unix_mode: value.unix_mode,
        }
    }
}

/// Get information about an open file descriptor. If the fd is invalid or closed, returns None.
pub fn twz_rt_fd_get_info(fd: RawFd) -> Option<FdInfo> {
    let mut info = core::mem::MaybeUninit::uninit();
    unsafe {
        if crate::bindings::twz_rt_fd_get_info(fd, info.as_mut_ptr()) {
            return Some(info.assume_init().into());
        }
    }
    None
}

pub use crate::bindings::name_entry as NameEntry;

impl Default for NameEntry {
    fn default() -> Self {
        Self {
            name: [0; crate::bindings::NAME_MAX as usize],
            name_len: 0,
            linkname_len: 0,
            info: FdInfo::default().into(),
        }
    }
}

impl NameEntry {
    pub fn new(iname: &[u8], info: crate::bindings::fd_info) -> Self {
        let nl = iname.len().min(crate::bindings::NAME_MAX as usize);
        let mut name = [0; crate::bindings::NAME_MAX as usize];
        name[0..nl].copy_from_slice(&iname[0..nl]);
        Self {
            name,
            info,
            name_len: nl as u32,
            linkname_len: 0,
        }
    }

    pub fn new_symlink(iname: &[u8], ilinkname: &[u8], info: crate::bindings::fd_info) -> Self {
        let nl = iname.len().min(crate::bindings::NAME_MAX as usize);
        let linknl = ilinkname.len().min(crate::bindings::NAME_MAX as usize - nl);
        let mut name = [0; crate::bindings::NAME_MAX as usize];
        name[0..nl].copy_from_slice(&iname[0..nl]);
        name[nl..(nl + linknl)].copy_from_slice(&ilinkname[0..linknl]);
        Self {
            name,
            info,
            name_len: nl as u32,
            linkname_len: linknl as u32,
        }
    }

    pub fn name_bytes(&self) -> &[u8] {
        &self.name[0..self.name_len as usize]
    }

    pub fn linkname_bytes(&self) -> &[u8] {
        &self.name[self.name_len as usize..(self.name_len + self.linkname_len) as usize]
    }
}

/// Enumerate sub-names for an fd (e.g. directory entries). Returns Some(n) on success, None if no
/// names can be enumerated. Return of Some(n) indicates number of items read into the buffer, 0 if
/// end of name list. Offset argument specifies number of entries to skip.
pub fn twz_rt_fd_enumerate_names(
    fd: RawFd,
    entries: &mut [NameEntry],
    off: usize,
) -> Option<usize> {
    let res = unsafe {
        crate::bindings::twz_rt_fd_enumerate_names(fd, entries.as_mut_ptr(), entries.len(), off)
    };
    match res {
        -1 => None,
        _ => Some(res as usize),
    }
}

/// Open a file descriptor by name, as a C-string.
pub fn twz_rt_fd_copen(
    name: &core::ffi::CStr,
    create: crate::bindings::create_options,
    flags: u32,
) -> Result<RawFd, OpenError> {
    let info = crate::bindings::open_info {
        name: name.as_ptr().cast(),
        len: name.count_bytes(),
        create,
        flags,
    };
    unsafe {
        let result = crate::bindings::twz_rt_fd_open(info);
        if let Ok(err) = result.error.try_into() {
            return Err(err);
        }
        Ok(result.fd)
    }
}

/// Open a file descriptor by name, as a Rust-string.
pub fn twz_rt_fd_open(
    name: &str,
    create: crate::bindings::create_options,
    flags: u32,
) -> Result<RawFd, OpenError> {
    let info = crate::bindings::open_info {
        name: name.as_ptr().cast(),
        len: name.len(),
        create,
        flags,
    };
    unsafe {
        let result = crate::bindings::twz_rt_fd_open(info);
        if let Ok(err) = result.error.try_into() {
            return Err(err);
        }
        Ok(result.fd)
    }
}

/// Remove a name
pub fn twz_rt_fd_remove(name: &str) -> Result<(), OpenError> {
    unsafe {
        let result = crate::bindings::twz_rt_fd_remove(name.as_ptr().cast(), name.len());
        if let Ok(err) = result.try_into() {
            return Err(err);
        }
        Ok(())
    }
}

/// Make a new namespace
pub fn twz_rt_fd_mkns(name: &str) -> Result<(), OpenError> {
    unsafe {
        let result = crate::bindings::twz_rt_fd_mkns(name.as_ptr().cast(), name.len());
        if let Ok(err) = result.try_into() {
            return Err(err);
        }
        Ok(())
    }
}

/// Make a new symlink
pub fn twz_rt_fd_symlink(name: &str, target: &str) -> Result<(), OpenError> {
    unsafe {
        let result = crate::bindings::twz_rt_fd_symlink(
            name.as_ptr().cast(),
            name.len(),
            target.as_ptr().cast(),
            target.len(),
        );
        if let Ok(err) = result.try_into() {
            return Err(err);
        }
        Ok(())
    }
}

pub fn twz_rt_fd_readlink(name: &str, buf: &mut [u8]) -> Result<usize, OpenError> {
    let mut len: u64 = 0;
    unsafe {
        let result = crate::bindings::twz_rt_fd_readlink(
            name.as_ptr().cast(),
            name.len(),
            buf.as_mut_ptr().cast(),
            buf.len(),
            &mut len,
        );
        if let Ok(err) = result.try_into() {
            return Err(err);
        }
        Ok(len as usize)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u32)]
pub enum OpenAnonKind {
    Pipe = crate::bindings::open_anon_kind_AnonKind_Pipe,
    Socket = crate::bindings::open_anon_kind_AnonKind_Socket,
}

impl TryFrom<u32> for OpenAnonKind {
    type Error = ();

    fn try_from(val: u32) -> Result<OpenAnonKind, Self::Error> {
        match val {
            crate::bindings::open_anon_kind_AnonKind_Pipe => Ok(Self::Pipe),
            crate::bindings::open_anon_kind_AnonKind_Socket => Ok(Self::Socket),
            _ => Err(()),
        }
    }
}

impl From<OpenAnonKind> for u32 {
    fn from(val: OpenAnonKind) -> u32 {
        match val {
            OpenAnonKind::Pipe => crate::bindings::open_anon_kind_AnonKind_Pipe,
            OpenAnonKind::Socket => crate::bindings::open_anon_kind_AnonKind_Socket,
        }
    }
}

/// Open an anonymous file descriptor.
pub fn twz_rt_fd_open_anon(kind: OpenAnonKind, flags: u32) -> Result<RawFd, OpenError> {
    unsafe {
        let result = crate::bindings::twz_rt_fd_open_anon(kind.into(), flags);
        if let Ok(err) = result.error.try_into() {
            return Err(err);
        }
        Ok(result.fd)
    }
}

/// Duplicate a file descriptor.
pub fn twz_rt_fd_dup(fd: RawFd) -> Result<RawFd, OpenError> {
    let mut new_fd = core::mem::MaybeUninit::<RawFd>::uninit();
    unsafe {
        if crate::bindings::twz_rt_fd_cmd(
            fd,
            crate::bindings::FD_CMD_DUP,
            core::ptr::null_mut(),
            new_fd.as_mut_ptr().cast(),
        ) == crate::bindings::FD_CMD_SUCCESS
        {
            return Ok(new_fd.assume_init());
        }
    }
    Err(OpenError::Other)
}

/// Sync a file descriptor.
pub fn twz_rt_fd_sync(fd: RawFd) {
    unsafe {
        crate::bindings::twz_rt_fd_cmd(
            fd,
            crate::bindings::FD_CMD_SYNC,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
        );
    }
}

/// Truncate a file descriptor.
pub fn twz_rt_fd_truncate(fd: RawFd, mut len: u64) -> Result<(), ()> {
    unsafe {
        if crate::bindings::twz_rt_fd_cmd(
            fd,
            crate::bindings::FD_CMD_TRUNCATE,
            (&mut len as *mut u64).cast(),
            core::ptr::null_mut(),
        ) != crate::bindings::FD_CMD_SUCCESS
        {
            return Err(());
        }
    }
    Ok(())
}

/// Close a file descriptor. If the fd is already closed, or invalid, this function has no effect.
pub fn twz_rt_fd_close(fd: RawFd) {
    unsafe { crate::bindings::twz_rt_fd_close(fd) }
}
