//! Interface for objects and object handles.

use core::{
    ffi::c_void,
    fmt::{LowerHex, UpperHex},
    mem::MaybeUninit,
    sync::atomic::{AtomicU64, Ordering},
};

use crate::{
    bindings::LEN_MUL,
    error::{RawTwzError, TwzError},
    Result,
};

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
        /// Persist changes on flush.
        const PERSIST = crate::bindings::MAP_FLAG_PERSIST;
        /// Use runtime support for read stability.
        const INDIRECT = crate::bindings::MAP_FLAG_INDIRECT;
    }
}

#[allow(dead_code)]
impl ObjectHandle {
    fn refs(&self) -> *const AtomicU64 {
        self.0.runtime_info.cast()
    }

    /// Get a pointer to the start of object data.
    pub fn start(&self) -> *mut u8 {
        self.0.start.cast()
    }

    /// Get a pointer to the metadata structure.
    pub fn meta(&self) -> *mut MetaInfo {
        self.0.meta.cast()
    }

    /// Get a slice of metadata extensions
    pub fn meta_exts(&self) -> &[MetaExt] {
        unsafe {
            core::slice::from_raw_parts(
                self.0.meta.cast::<u8>().add(size_of::<MetaInfo>()).cast(),
                (*self.meta()).extcount as usize,
            )
        }
    }

    /// Find the first metadata extension matching the given tag
    pub fn find_meta_ext(&self, tag: MetaExtTag) -> Option<&MetaExt> {
        self.meta_exts().iter().find(|e| e.tag == tag)
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
impl From<Result<ObjectHandle>> for crate::bindings::map_result {
    fn from(value: Result<ObjectHandle>) -> Self {
        match value {
            Ok(handle) => Self {
                handle: handle.into_raw(),
                error: RawTwzError::success().raw(),
            },
            Err(e) => Self {
                handle: crate::bindings::object_handle::default(),
                error: e.raw(),
            },
        }
    }
}

#[cfg(not(feature = "kernel"))]
impl From<crate::bindings::map_result> for Result<ObjectHandle> {
    fn from(value: crate::bindings::map_result) -> Self {
        let raw = RawTwzError::new(value.error);
        if raw.is_success() {
            Ok(ObjectHandle(value.handle))
        } else {
            Err(raw.error())
        }
    }
}

/// Map an object given by ID `id` with the given flags.
#[cfg(not(feature = "kernel"))]
pub fn twz_rt_map_object(id: ObjID, flags: MapFlags) -> Result<ObjectHandle> {
    unsafe { crate::bindings::twz_rt_map_object(id.raw(), flags.bits()).into() }
}

#[cfg(not(feature = "kernel"))]
pub fn twz_rt_get_object_handle(ptr: *const u8) -> Result<ObjectHandle> {
    use crate::error::ObjectError;

    let res = unsafe { crate::bindings::twz_rt_get_object_handle((ptr as *mut u8).cast()) };
    if res.id == 0 {
        return Err(TwzError::Object(ObjectError::NotMapped));
    }
    Ok(ObjectHandle(res))
}

#[cfg(not(feature = "kernel"))]
pub fn twz_rt_resolve_fot(this: &ObjectHandle, idx: u64, valid_len: usize) -> Result<ObjectHandle> {
    unsafe {
        crate::bindings::twz_rt_resolve_fot(&this.0 as *const _ as *mut _, idx, valid_len).into()
    }
}

impl From<Result<u32>> for crate::bindings::u32_result {
    fn from(value: Result<u32>) -> Self {
        match value {
            Ok(val) => Self {
                val,
                err: RawTwzError::success().raw(),
            },
            Err(e) => Self {
                val: 0,
                err: e.raw(),
            },
        }
    }
}

impl From<crate::bindings::u32_result> for Result<u32> {
    fn from(value: crate::bindings::u32_result) -> Self {
        let raw = RawTwzError::new(value.err);
        if raw.is_success() {
            Ok(value.val)
        } else {
            Err(raw.error())
        }
    }
}

#[cfg(not(feature = "kernel"))]
pub fn twz_rt_insert_fot(this: &ObjectHandle, entry: *const u8) -> Result<u32> {
    unsafe {
        let res = crate::bindings::twz_rt_insert_fot(
            &this.0 as *const _ as *mut _,
            (entry as *mut u8).cast(),
        );
        res.into()
    }
}

#[cfg(not(feature = "kernel"))]
pub fn twz_rt_resolve_fot_local(start: *mut u8, idx: u64, valid_len: usize) -> *mut u8 {
    unsafe {
        let res = crate::bindings::twz_rt_resolve_fot_local(start.cast(), idx, valid_len);
        res.cast()
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
) -> Result<(ObjectHandle, ObjectHandle)> {
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

        let res1: Result<ObjectHandle> = res1.into();
        let res2: Result<ObjectHandle> = res2.into();

        Ok((res1?, res2?))
    }
}

/// Flags for objects.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq, Hash, Default)]
#[repr(transparent)]
pub struct MetaFlags(pub u32);

/// A nonce for avoiding object ID collision.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq, Hash, Default)]
#[repr(transparent)]
pub struct Nonce(pub u128);

/// The core metadata that all objects share.
#[repr(C)]
pub struct MetaInfo {
    /// The ID nonce.
    pub nonce: Nonce,
    /// The object's public key ID.
    pub kuid: ObjID,
    /// The object flags.
    pub flags: MetaFlags,
    /// The number of FOT entries.
    pub fotcount: u16,
    /// The number of meta extensions.
    pub extcount: u16,
}

/// A tag for a meta extension entry.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[repr(transparent)]
pub struct MetaExtTag(pub u64);

/// A meta extension entry.
#[repr(C)]
pub struct MetaExt {
    /// The tag.
    pub tag: MetaExtTag,
    /// A tag-specific value.
    pub value: u64,
}

pub const MEXT_EMPTY: MetaExtTag = MetaExtTag(0);
pub const MEXT_SIZED: MetaExtTag = MetaExtTag(1);
