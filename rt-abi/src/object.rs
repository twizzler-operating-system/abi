#![allow(unused_variables)]

use core::sync::atomic::{AtomicU64, Ordering};

pub use twizzler_types::ObjID;

pub struct ObjectHandle(pub(crate) crate::bindings::object_handle);

bitflags::bitflags! {
    pub struct MapFlags : u32 {
        const MAP_READ = crate::bindings::MAP_FLAG_R;
        const MAP_WRITE = crate::bindings::MAP_FLAG_W;
        const MAP_EXEC = crate::bindings::MAP_FLAG_X;
    }
}

#[repr(u32)]
pub enum MapError {
    Other = crate::bindings::map_error_MapError_Other,
    OutOfResources = crate::bindings::map_error_MapError_OutOfResources,
    NoSuchObject = crate::bindings::map_error_MapError_NoSuchObject,
    PermissionDenied = crate::bindings::map_error_MapError_PermissionDenied,
    InvalidArgument = crate::bindings::map_error_MapError_InvalidArgument,
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

pub fn twz_rt_map_object(id: ObjID, flags: MapFlags) -> Result<ObjectHandle, MapError> {
    todo!()
}

pub fn twz_rt_release_handle(handle: &mut ObjectHandle) {
    todo!()
}

#[deprecated]
pub fn twz_rt_map_two_objects(id1: ObjID, flags1: MapFlags, id2: ObjID, flags2: ObjID) -> Result<(ObjectHandle, ObjectHandle), MapError> {
    todo!()
}
