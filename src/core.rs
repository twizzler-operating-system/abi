pub fn twz_rt_exit(code: i32) -> ! {
    unsafe {
        crate::bindings::twz_rt_exit(code);
        unreachable!()
    }
}

pub fn twz_rt_abort() -> ! {
    unsafe {
        crate::bindings::twz_rt_abort();
        unreachable!()
    }
}

pub fn twz_rt_pre_main_hook() -> Option<i32> {
    unsafe {
        crate::bindings::twz_rt_pre_main_hook().into()
    }
}

impl From<crate::bindings::option_i32> for Option<i32> {
    #[inline]
    fn from(value: crate::bindings::option_i32) -> Self {
        if value.is_some == 0 {
            None
        } else {
            Some(value.value)
        }
    }
}

pub fn twz_rt_post_main_hook() {
    unsafe {
        crate::bindings::twz_rt_post_main_hook();
    }
}

pub use crate::bindings::basic_aux as BasicAux;
pub use crate::bindings::basic_return as BasicReturn;
pub use crate::bindings::runtime_info as RuntimeInfo;
pub fn twz_rt_runtime_entry(info: *const RuntimeInfo, std_entry: unsafe extern "C-unwind" fn(BasicAux) -> BasicReturn) -> ! {
    unsafe {
        crate::bindings::twz_rt_runtime_entry(info, Some(std_entry));
        unreachable!()
    }
}
