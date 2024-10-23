
pub type DlPhdrInfo = crate::bindings::dl_phdr_info;
pub type LoadedImage = crate::bindings::loaded_image;

pub fn twz_rt_get_loaded_image(_id: u32) -> Option<LoadedImage> {
    todo!()
}

pub fn twz_rt_iter_phdr(_f: &mut dyn Fn(DlPhdrInfo) -> i32) -> i32 {
    todo!()
}
