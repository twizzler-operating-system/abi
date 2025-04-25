//! Runtime interface for file descriptors.

use core::time::Duration;

pub use crate::bindings::descriptor as RawFd;
use crate::{
    error::{ArgumentError, RawTwzError, TwzError},
    Result,
};

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

impl From<Result<RawFd>> for crate::bindings::open_result {
    fn from(value: Result<RawFd>) -> Self {
        match value {
            Ok(fd) => Self {
                fd,
                err: RawTwzError::success().raw(),
            },
            Err(e) => Self {
                fd: 0,
                err: e.raw(),
            },
        }
    }
}

impl From<crate::bindings::open_result> for Result<RawFd> {
    fn from(value: crate::bindings::open_result) -> Self {
        let raw = RawTwzError::new(value.err);
        if raw.is_success() {
            Ok(value.fd)
        } else {
            Err(raw.error())
        }
    }
}

/// Get information about an open file descriptor. If the fd is invalid or closed, returns None.
pub fn twz_rt_fd_get_info(fd: RawFd) -> Result<FdInfo> {
    let mut info = core::mem::MaybeUninit::uninit();
    unsafe {
        if crate::bindings::twz_rt_fd_get_info(fd, info.as_mut_ptr()) {
            return Ok(info.assume_init().into());
        }
    }
    Err(TwzError::Argument(ArgumentError::BadHandle))
}

pub use crate::bindings::name_entry as NameEntry;

impl Default for NameEntry {
    fn default() -> Self {
        Self {
            name: [0; Self::NAME_MAX_LEN],
            name_len: 0,
            linkname_len: 0,
            info: FdInfo::default().into(),
        }
    }
}

impl NameEntry {
    pub const NAME_MAX_LEN: usize = crate::bindings::NAME_ENTRY_LEN as usize;
    pub fn new(iname: &[u8], info: crate::bindings::fd_info) -> Self {
        let nl = iname.len().min(Self::NAME_MAX_LEN);
        let mut name = [0; Self::NAME_MAX_LEN];
        name[0..nl].copy_from_slice(&iname[0..nl]);
        Self {
            name,
            info,
            name_len: nl as u32,
            linkname_len: 0,
        }
    }

    pub fn new_symlink(iname: &[u8], ilinkname: &[u8], info: crate::bindings::fd_info) -> Self {
        let nl = iname.len().min(Self::NAME_MAX_LEN);
        let linknl = ilinkname.len().min(Self::NAME_MAX_LEN - nl);
        let mut name = [0; Self::NAME_MAX_LEN];
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
) -> Result<usize> {
    unsafe {
        crate::bindings::twz_rt_fd_enumerate_names(fd, entries.as_mut_ptr(), entries.len(), off)
            .into()
    }
}

/// Open a file descriptor by name, as a C-string.
pub fn twz_rt_fd_copen(
    name: &core::ffi::CStr,
    create: crate::bindings::create_options,
    flags: u32,
) -> Result<RawFd> {
    let info = crate::bindings::open_info {
        name: name.as_ptr().cast(),
        len: name.count_bytes(),
        create,
        flags,
    };
    unsafe { crate::bindings::twz_rt_fd_open(info).into() }
}

/// Open a file descriptor by name, as a Rust-string.
pub fn twz_rt_fd_open(
    name: &str,
    create: crate::bindings::create_options,
    flags: u32,
) -> Result<RawFd> {
    let info = crate::bindings::open_info {
        name: name.as_ptr().cast(),
        len: name.len(),
        create,
        flags,
    };
    unsafe { crate::bindings::twz_rt_fd_open(info).into() }
}

/// Remove a name
pub fn twz_rt_fd_remove(name: &str) -> Result<()> {
    unsafe {
        RawTwzError::new(crate::bindings::twz_rt_fd_remove(
            name.as_ptr().cast(),
            name.len(),
        ))
        .result()
    }
}

/// Make a new namespace
pub fn twz_rt_fd_mkns(name: &str) -> Result<()> {
    unsafe {
        RawTwzError::new(crate::bindings::twz_rt_fd_mkns(
            name.as_ptr().cast(),
            name.len(),
        ))
        .result()
    }
}

/// Make a new symlink
pub fn twz_rt_fd_symlink(name: &str, target: &str) -> Result<()> {
    unsafe {
        RawTwzError::new(crate::bindings::twz_rt_fd_symlink(
            name.as_ptr().cast(),
            name.len(),
            target.as_ptr().cast(),
            target.len(),
        ))
        .result()
    }
}

pub fn twz_rt_fd_readlink(name: &str, buf: &mut [u8]) -> Result<usize> {
    let mut len: u64 = 0;
    unsafe {
        RawTwzError::new(crate::bindings::twz_rt_fd_readlink(
            name.as_ptr().cast(),
            name.len(),
            buf.as_mut_ptr().cast(),
            buf.len(),
            &mut len,
        ))
        .result()?;
    }
    Ok(len as usize)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u32)]
pub enum OpenAnonKind {
    Pipe = crate::bindings::open_anon_kind_AnonKind_Pipe,
    SocketConnect = crate::bindings::open_anon_kind_AnonKind_SocketConnect,
    SocketBind = crate::bindings::open_anon_kind_AnonKind_SocketBind,
}

impl TryFrom<u32> for OpenAnonKind {
    type Error = ();

    fn try_from(val: u32) -> core::result::Result<OpenAnonKind, Self::Error> {
        match val {
            crate::bindings::open_anon_kind_AnonKind_Pipe => Ok(Self::Pipe),
            crate::bindings::open_anon_kind_AnonKind_SocketConnect => Ok(Self::SocketConnect),
            crate::bindings::open_anon_kind_AnonKind_SocketBind => Ok(Self::SocketBind),
            _ => Err(()),
        }
    }
}

impl From<OpenAnonKind> for u32 {
    fn from(val: OpenAnonKind) -> u32 {
        match val {
            OpenAnonKind::Pipe => crate::bindings::open_anon_kind_AnonKind_Pipe,
            OpenAnonKind::SocketBind => crate::bindings::open_anon_kind_AnonKind_SocketBind,
            OpenAnonKind::SocketConnect => crate::bindings::open_anon_kind_AnonKind_SocketConnect,
        }
    }
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct SocketAddress(pub(crate) crate::bindings::socket_address);

impl SocketAddress {
    fn new_v4(octets: [u8; 4], port: u16) -> Self {
        Self(crate::bindings::socket_address {
            kind: crate::bindings::addr_kind_AddrKind_Ipv4,
            addr_octets: crate::bindings::socket_address_addrs { v4: octets },
            port,
            flowinfo: 0,
            scope_id: 0,
        })
    }

    fn new_v6(octets: [u8; 16], port: u16, flowinfo: u32, scope_id: u32) -> Self {
        Self(crate::bindings::socket_address {
            kind: crate::bindings::addr_kind_AddrKind_Ipv6,
            addr_octets: crate::bindings::socket_address_addrs { v6: octets },
            port,
            flowinfo,
            scope_id,
        })
    }

    fn v4_octets(&self) -> [u8; 4] {
        assert!(self.is_v4());
        unsafe { self.0.addr_octets.v4 }
    }

    fn v6_octets(&self) -> [u8; 16] {
        assert!(!self.is_v4());
        unsafe { self.0.addr_octets.v6 }
    }

    fn is_v4(&self) -> bool {
        self.0.kind == crate::bindings::addr_kind_AddrKind_Ipv4
    }
}

impl From<SocketAddress> for core::net::IpAddr {
    fn from(value: SocketAddress) -> Self {
        if value.is_v4() {
            Self::V4(core::net::Ipv4Addr::from_octets(value.v4_octets()))
        } else {
            Self::V6(core::net::Ipv6Addr::from_octets(value.v6_octets()))
        }
    }
}

impl From<SocketAddress> for core::net::SocketAddr {
    fn from(value: SocketAddress) -> Self {
        if value.is_v4() {
            Self::V4(core::net::SocketAddrV4::new(
                core::net::Ipv4Addr::from_octets(value.v4_octets()),
                value.0.port,
            ))
        } else {
            Self::V6(core::net::SocketAddrV6::new(
                core::net::Ipv6Addr::from_octets(value.v6_octets()),
                value.0.port,
                value.0.flowinfo,
                value.0.scope_id,
            ))
        }
    }
}

impl From<core::net::Ipv4Addr> for SocketAddress {
    fn from(value: core::net::Ipv4Addr) -> Self {
        Self::new_v4(value.octets(), 0)
    }
}

impl From<core::net::Ipv6Addr> for SocketAddress {
    fn from(value: core::net::Ipv6Addr) -> Self {
        Self::new_v6(value.octets(), 0, 0, 0)
    }
}

impl From<core::net::SocketAddrV4> for SocketAddress {
    fn from(value: core::net::SocketAddrV4) -> Self {
        Self::new_v4(value.ip().octets(), value.port())
    }
}

impl From<core::net::SocketAddrV6> for SocketAddress {
    fn from(value: core::net::SocketAddrV6) -> Self {
        Self::new_v6(
            value.ip().octets(),
            value.port(),
            value.flowinfo(),
            value.scope_id(),
        )
    }
}

impl From<core::net::SocketAddr> for SocketAddress {
    fn from(value: core::net::SocketAddr) -> Self {
        match value {
            core::net::SocketAddr::V4(v4) => v4.into(),
            core::net::SocketAddr::V6(v6) => v6.into(),
        }
    }
}

impl From<core::net::IpAddr> for SocketAddress {
    fn from(value: core::net::IpAddr) -> Self {
        match value {
            core::net::IpAddr::V4(v4) => v4.into(),
            core::net::IpAddr::V6(v6) => v6.into(),
        }
    }
}

/// Open an anonymous file descriptor.
pub fn twz_rt_fd_open_socket_bind(mut addr: SocketAddress, flags: u32) -> Result<RawFd> {
    unsafe {
        crate::bindings::twz_rt_fd_open_anon(
            OpenAnonKind::SocketBind.into(),
            flags,
            ((&mut addr.0) as *mut crate::bindings::socket_address).cast(),
            core::mem::size_of::<crate::bindings::socket_address>(),
        )
        .into()
    }
}

/// Open an anonymous file descriptor.
pub fn twz_rt_fd_open_socket_connect(mut addr: SocketAddress, flags: u32) -> Result<RawFd> {
    unsafe {
        crate::bindings::twz_rt_fd_open_anon(
            OpenAnonKind::SocketConnect.into(),
            flags,
            ((&mut addr.0) as *mut crate::bindings::socket_address).cast(),
            core::mem::size_of::<crate::bindings::socket_address>(),
        )
        .into()
    }
}

/// Open an anonymous file descriptor.
pub fn twz_rt_fd_open_pipe(flags: u32) -> Result<RawFd> {
    unsafe {
        crate::bindings::twz_rt_fd_open_anon(
            OpenAnonKind::Pipe.into(),
            flags,
            core::ptr::null_mut(),
            0,
        )
        .into()
    }
}

/// Duplicate a file descriptor.
pub fn twz_rt_fd_dup(fd: RawFd) -> Result<RawFd> {
    let mut new_fd = core::mem::MaybeUninit::<RawFd>::uninit();
    unsafe {
        let e = crate::bindings::twz_rt_fd_cmd(
            fd,
            crate::bindings::FD_CMD_DUP,
            core::ptr::null_mut(),
            new_fd.as_mut_ptr().cast(),
        );
        let raw = RawTwzError::new(e);
        if raw.is_success() {
            Ok(new_fd.assume_init())
        } else {
            Err(raw.error())
        }
    }
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
pub fn twz_rt_fd_truncate(fd: RawFd, mut len: u64) -> Result<()> {
    unsafe {
        let e = crate::bindings::twz_rt_fd_cmd(
            fd,
            crate::bindings::FD_CMD_TRUNCATE,
            (&mut len as *mut u64).cast(),
            core::ptr::null_mut(),
        );
        let raw = RawTwzError::new(e);
        if !raw.is_success() {
            return Err(raw.error());
        }
    }
    Ok(())
}

/// Close a file descriptor. If the fd is already closed, or invalid, this function has no effect.
pub fn twz_rt_fd_close(fd: RawFd) {
    unsafe { crate::bindings::twz_rt_fd_close(fd) }
}
