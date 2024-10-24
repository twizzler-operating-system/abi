use crate::object::ObjectHandle;

pub type DlPhdrInfo = crate::bindings::dl_phdr_info;
pub type LoadedImageId = crate::bindings::loaded_image_id;
pub struct LoadedImage(crate::bindings::loaded_image);

pub use crate::bindings::TWZ_RT_EXEID;

impl LoadedImage {
    pub fn image(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(self.0.image_start.cast(), self.0.image_len)
        }
    }

    pub fn handle(&self) -> ObjectHandle {
        todo!()
    }
    
    pub fn id(&self) -> LoadedImageId {
        self.0.id
    }

    pub fn dl_info(&self) -> &DlPhdrInfo {
        &self.0.dl_info
    }
}

impl Clone for LoadedImage {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl Drop for LoadedImage {
    fn drop(&mut self) {
        todo!()
    }
}

pub fn twz_rt_get_loaded_image(_id: LoadedImageId) -> Option<LoadedImage> {
    todo!()
}

pub fn twz_rt_iter_phdr(_f: &mut dyn Fn(DlPhdrInfo) -> i32) -> i32 {
    todo!()
}
