
pub type DlPhdrInfo = crate::bindings::dl_phdr_info;
pub type Library = crate::bindings::dso;

pub fn twz_rt_get_library(_id: u32) -> Option<Library> {
    todo!()
}

pub fn twz_rt_iter_phdr(_f: &mut dyn Fn(DlPhdrInfo) -> i32) -> i32 {
    todo!()
}
