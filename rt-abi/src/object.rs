#![allow(unused_variables)]

use core::sync::atomic::{AtomicU64, Ordering};
use core::mem::MaybeUninit;
use core::fmt::{UpperHex, LowerHex};

/// An object ID.
#[repr(transparent)]
pub struct ObjID(twizzler_types::ObjID);

impl ObjID {
    /// The number of u64 components that make up an object ID, if split.
    pub const NR_PARTS: usize = 2;

    /// Build a new object ID from raw.
    pub fn new(raw: twizzler_types::ObjID) -> Self {
        Self(raw)
    }

    /// Get the raw object ID type.
    pub fn raw(&self) -> twizzler_types::ObjID {
        self.0
    }

    /// Build an object ID from parts, useful for syscalls.
    pub fn from_parts(parts: [u64; Self::NR_PARTS]) -> Self {
        Self::new(((parts[0] as u128) << 64) | (parts[1] as u128))
    }

    /// Split the object ID into parts, useful for packing into registers for syscalls.
    pub fn parts(&self) -> [u64; Self::NR_PARTS] {
        [(self.0 >> 64) as u64, (self.0 & 0xffffffffffffffff) as u64]
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
///   - Owning -- the normal mode, which acts like an Arc, and asks the runtime to unmap when refcount hits zero.
///   - Unsafe -- internal use only, is NOT owning, but still has pointers. This is totally unsafe to use, and
///               should not be exposed to users. But sometimes, it can be safe, and faster than cloning.
/// ... anyway, in general these have reference counting semantics, via Clone and Drop, like Arc.
#[repr(transparent)]
pub struct ObjectHandle(pub(crate) crate::bindings::object_handle);

bitflags::bitflags! {
    /// Flags for mapping objects.
    pub struct MapFlags : crate::bindings::map_flags {
        /// Request READ access.
        const MAP_READ = crate::bindings::MAP_FLAG_R;
        /// Request WRITE access.
        const MAP_WRITE = crate::bindings::MAP_FLAG_W;
        /// Request EXECUTE access.
        const MAP_EXEC = crate::bindings::MAP_FLAG_X;
    }
}

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

impl TryFrom<crate::bindings::map_error> for MapError {
    type Error = ();
    fn try_from(value: crate::bindings::map_error) -> Result<Self, Self::Error> {
        match value {
            crate::bindings::map_error_MapError_Success => Err(()),
            crate::bindings::map_error_MapError_OutOfResources => Ok(Self::OutOfResources),
            crate::bindings::map_error_MapError_NoSuchObject => Ok(Self::NoSuchObject),
            crate::bindings::map_error_MapError_PermissionDenied => Ok(Self::PermissionDenied),
            crate::bindings::map_error_MapError_InvalidArgument => Ok(Self::InvalidArgument),
            _ => Ok(Self::Other)
        }
    }
}

impl ObjectHandle {
    fn refs(&self) -> *const AtomicU64 {
        self.0.runtime_info.cast()
    }
}

impl Clone for ObjectHandle {
    fn clone(&self) -> Self {
        unsafe {       
            let Some(ref rc) = self.refs().as_ref() else {
                panic!("cannot clone an unsafe object handle");
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

/// Map an object given by ID `id` with the given flags.
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
pub fn twz_rt_release_handle(handle: &mut ObjectHandle) {
    unsafe { crate::bindings::twz_rt_release_handle(&mut handle.0) }
}

#[deprecated]
pub fn twz_rt_map_two_objects(id1: ObjID, flags1: MapFlags, id2: ObjID, flags2: MapFlags) -> Result<(ObjectHandle, ObjectHandle), MapError> {
    unsafe {
        let mut res1 = MaybeUninit::uninit();
        let mut res2 = MaybeUninit::uninit();
        crate::bindings::__twz_rt_map_two_objects(id1.raw(), flags1.bits(), id2.raw(), flags2.bits(), res1.as_mut_ptr(), res2.as_mut_ptr());

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
