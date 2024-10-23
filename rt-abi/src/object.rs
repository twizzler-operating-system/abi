#![allow(unused_variables)]

pub use twizzler_types::ObjID;

pub type ObjectHandle = crate::bindings::object_handle;

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
