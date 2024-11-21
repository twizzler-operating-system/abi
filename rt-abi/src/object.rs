//! Interface for objects and object handles.

use core::{
    fmt::{LowerHex, UpperHex},
    mem::MaybeUninit,
    sync::atomic::{AtomicU64, Ordering},
};

use crate::bindings::LEN_MUL;

/// An object ID.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
#[repr(transparent)]
pub struct ObjID(twizzler_types::ObjID);

impl ObjID {
    /// The number of u64 components that make up an object ID, if split.
    pub const NR_PARTS: usize = 2;

    /// Build a new object ID from raw.
    pub const fn new(raw: twizzler_types::ObjID) -> Self {
        Self(raw)
    }

    /// Get the raw object ID type.
    pub const fn raw(&self) -> twizzler_types::ObjID {
        self.0
    }

    /// Build an object ID from parts, useful for syscalls.
    pub const fn from_parts(parts: [u64; Self::NR_PARTS]) -> Self {
        Self::new(((parts[0] as u128) << 64) | (parts[1] as u128))
    }

    /// Split the object ID into parts, useful for packing into registers for syscalls.
    pub const fn parts(&self) -> [u64; Self::NR_PARTS] {
        [(self.0 >> 64) as u64, (self.0 & 0xffffffffffffffff) as u64]
    }

    #[deprecated]
    pub const fn split(&self) -> (u64, u64) {
        let parts = self.parts();
        (parts[0], parts[1])
    }

    #[deprecated]
    pub const fn new_from_parts(hi: u64, lo: u64) -> Self {
        Self::from_parts([hi, lo])
    }

    #[deprecated]
    /// Read the raw value.
    pub const fn as_u128(&self) -> u128 {
        self.0
    }
}

impl core::convert::AsRef<ObjID> for ObjID {
    fn as_ref(&self) -> &ObjID {
        self
    }
}

impl From<twizzler_types::ObjID> for ObjID {
    fn from(id: twizzler_types::ObjID) -> Self {
        Self::new(id)
    }
}

impl LowerHex for ObjID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:x}", self.0)
    }
}

impl UpperHex for ObjID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:X}", self.0)
    }
}

impl core::fmt::Display for ObjID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ObjID({:x})", self.0)
    }
}

impl core::fmt::Debug for ObjID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ObjID({:x})", self.0)
    }
}

/// An object handle, granting access to object memory. An object handle can be in two modes:
///   - Owning -- the normal mode, which acts like an Arc, and asks the runtime to unmap when
///     refcount hits zero.
///   - Unsafe -- internal use only, is NOT owning, but still has pointers. This is totally unsafe
///     to use, and should not be exposed to users. But sometimes, it can be safe, and faster than
///     cloning.
/// ... anyway, in general these have reference counting semantics, via Clone and Drop, like Arc.
#[repr(transparent)]
pub struct ObjectHandle(pub(crate) crate::bindings::object_handle);

#[cfg(not(feature = "kernel"))]
impl core::fmt::Debug for ObjectHandle {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "ObjectHandle({:?}, {:p}, {:x}, {:?})",
            self.id(),
            self.start(),
            self.valid_len(),
            self.map_flags()
        )
    }
}

unsafe impl Send for ObjectHandle {}
unsafe impl Sync for ObjectHandle {}

bitflags::bitflags! {
    /// Flags for mapping objects.
    #[derive(Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord, Debug)]
    pub struct MapFlags : crate::bindings::map_flags {
        /// Request READ access.
        const READ = crate::bindings::MAP_FLAG_R;
        /// Request WRITE access.
        const WRITE = crate::bindings::MAP_FLAG_W;
        /// Request EXECUTE access.
        const EXEC = crate::bindings::MAP_FLAG_X;
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u32)]
/// Possible errors for mapping objects.
pub enum MapError {
    /// Unclassified error.
    Other = crate::bindings::map_error_MapError_Other,
    /// Out of resources (e.g. mapping slots)
    OutOfResources = crate::bindings::map_error_MapError_OutOfResources,
    /// Specified object was not found
    NoSuchObject = crate::bindings::map_error_MapError_NoSuchObject,
    /// Permission denied
    PermissionDenied = crate::bindings::map_error_MapError_PermissionDenied,
    /// An argument to map was invalid
    InvalidArgument = crate::bindings::map_error_MapError_InvalidArgument,
}

impl core::fmt::Display for MapError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            MapError::Other => write!(f, "unclassified error"),
            MapError::OutOfResources => write!(f, "out of resources"),
            MapError::NoSuchObject => write!(f, "no such object"),
            MapError::PermissionDenied => write!(f, "permission denied"),
            MapError::InvalidArgument => write!(f, "invalid argument"),
        }
    }
}

impl core::error::Error for MapError {}

impl TryFrom<crate::bindings::map_error> for MapError {
    type Error = ();
    fn try_from(value: crate::bindings::map_error) -> Result<Self, Self::Error> {
        match value {
            crate::bindings::map_error_MapError_Success => Err(()),
            crate::bindings::map_error_MapError_OutOfResources => Ok(Self::OutOfResources),
            crate::bindings::map_error_MapError_NoSuchObject => Ok(Self::NoSuchObject),
            crate::bindings::map_error_MapError_PermissionDenied => Ok(Self::PermissionDenied),
            crate::bindings::map_error_MapError_InvalidArgument => Ok(Self::InvalidArgument),
            _ => Ok(Self::Other),
        }
    }
}

impl From<MapError> for crate::bindings::map_error {
    fn from(value: MapError) -> Self {
        value as Self
    }
}

#[cfg(not(feature = "kernel"))]
impl ObjectHandle {
    fn refs(&self) -> *const AtomicU64 {
        self.0.runtime_info.cast()
    }

    /// Get a pointer to the start of object data.
    pub fn start(&self) -> *mut u8 {
        self.0.start.cast()
    }

    /// Get a pointer to the metadata structure.
    pub fn meta(&self) -> *mut u8 {
        self.0.meta.cast()
    }

    /// Get a pointer to the runtime info.
    pub fn runtime_info(&self) -> *mut u8 {
        self.0.runtime_info.cast()
    }

    /// Get map flags.
    pub fn map_flags(&self) -> MapFlags {
        MapFlags::from_bits_truncate(self.0.map_flags)
    }

    /// Get the number of valid bytes after start pointer for object data.
    pub fn valid_len(&self) -> usize {
        (self.0.valid_len as usize) * crate::bindings::LEN_MUL
    }

    /// Get the object ID.
    pub fn id(&self) -> ObjID {
        ObjID::new(self.0.id)
    }

    /// Get the raw object handle.
    pub fn into_raw(self) -> crate::bindings::object_handle {
        let this = core::mem::ManuallyDrop::new(self);
        this.0
    }

    /// Build an object handle from raw.
    pub fn from_raw(raw: crate::bindings::object_handle) -> Self {
        Self(raw)
    }

    /// Make a new object handle.
    ///
    /// # Safety
    /// The caller must ensure that runtime_info is a valid pointer, and points to a repr(C) struct
    /// that starts with an AtomicU64 for reference counting.
    pub unsafe fn new(
        id: ObjID,
        runtime_info: *mut core::ffi::c_void,
        start: *mut core::ffi::c_void,
        meta: *mut core::ffi::c_void,
        map_flags: MapFlags,
        valid_len: usize,
    ) -> Self {
        Self::from_raw(crate::bindings::object_handle {
            id: id.0,
            runtime_info,
            start,
            meta,
            map_flags: map_flags.bits(),
            valid_len: (valid_len / LEN_MUL) as u32,
        })
    }
}

#[cfg(not(feature = "kernel"))]
impl Clone for ObjectHandle {
    fn clone(&self) -> Self {
        unsafe {
            let Some(ref rc) = self.refs().as_ref() else {
                return Self(self.0);
            };
            // This use of Relaxed ordering is justified by https://doc.rust-lang.org/nomicon/arc-mutex/arc-clone.html.
            let old_count = rc.fetch_add(1, Ordering::Relaxed);
            // The above link also justifies the following behavior. If our count gets this high, we
            // have probably run into a problem somewhere.
            if old_count >= i64::MAX as u64 {
                crate::core::twz_rt_abort();
            }
        }
        Self(self.0)
    }
}

#[cfg(not(feature = "kernel"))]
impl Drop for ObjectHandle {
    fn drop(&mut self) {
        unsafe {
            let Some(ref rc) = self.refs().as_ref() else {
                return;
            };
            // This use of Release ordering is justified by https://doc.rust-lang.org/nomicon/arc-mutex/arc-clone.html.
            if rc.fetch_sub(1, Ordering::Release) != 1 {
                return;
            }
        }
        // This fence is needed to prevent reordering of the use and deletion
        // of the data.
        core::sync::atomic::fence(Ordering::Acquire);
        twz_rt_release_handle(self);
    }
}

impl Default for crate::bindings::object_handle {
    fn default() -> Self {
        Self {
            id: 0,
            map_flags: 0,
            start: core::ptr::null_mut(),
            meta: core::ptr::null_mut(),
            runtime_info: core::ptr::null_mut(),
            valid_len: 0,
        }
    }
}

#[cfg(not(feature = "kernel"))]
impl From<Result<ObjectHandle, MapError>> for crate::bindings::map_result {
    fn from(value: Result<ObjectHandle, MapError>) -> Self {
        match value {
            Ok(handle) => Self {
                handle: handle.into_raw(),
                error: crate::bindings::map_error_MapError_Success,
            },
            Err(e) => Self {
                handle: crate::bindings::object_handle::default(),
                error: e.into(),
            },
        }
    }
}

/// Map an object given by ID `id` with the given flags.
#[cfg(not(feature = "kernel"))]
pub fn twz_rt_map_object(id: ObjID, flags: MapFlags) -> Result<ObjectHandle, MapError> {
    unsafe {
        let res = crate::bindings::twz_rt_map_object(id.raw(), flags.bits());
        if let Ok(map_error) = res.error.try_into() {
            return Err(map_error);
        }

        Ok(ObjectHandle(res.handle))
    }
}

/// Release a handle. Should be only called by the ObjectHandle drop call.
#[cfg(not(feature = "kernel"))]
pub fn twz_rt_release_handle(handle: &mut ObjectHandle) {
    unsafe { crate::bindings::twz_rt_release_handle(&mut handle.0) }
}

#[deprecated]
#[cfg(not(feature = "kernel"))]
pub fn twz_rt_map_two_objects(
    id1: ObjID,
    flags1: MapFlags,
    id2: ObjID,
    flags2: MapFlags,
) -> Result<(ObjectHandle, ObjectHandle), MapError> {
    unsafe {
        let mut res1 = MaybeUninit::uninit();
        let mut res2 = MaybeUninit::uninit();
        crate::bindings::__twz_rt_map_two_objects(
            id1.raw(),
            flags1.bits(),
            id2.raw(),
            flags2.bits(),
            res1.as_mut_ptr(),
            res2.as_mut_ptr(),
        );

        let res1 = res1.assume_init();
        let res2 = res2.assume_init();

        let me1 = res1.error.try_into();
        let me2 = res2.error.try_into();

        if let Ok(map_error) = me1 {
            if me2.is_err() {
                // This means res2 DOES have an object handle.
                let _handle = ObjectHandle(res2.handle);
            }
            return Err(map_error);
        }

        if let Ok(map_error) = me2 {
            if me1.is_err() {
                // This means res1 DOES have an object handle.
                let _handle = ObjectHandle(res1.handle);
            }
            return Err(map_error);
        }

        Ok((ObjectHandle(res1.handle), ObjectHandle(res2.handle)))
    }
}
