
pub type ObjID = crate::bindings::rt_objid;

bitflags::bitflags! {
    pub struct MapFlags : u32 {
        const MAP_READ = crate::bindings::MAP_FLAG_R;
        const MAP_WRITE = crate::bindings::MAP_FLAG_W;
        const MAP_EXEC = crate::bindings::MAP_FLAG_X;
    }
}

pub fn twz_rt_map_object(id: ObjID, flags: MapFlags) -> Result<ObjectHandle, MapError> {
    todo!()
}
