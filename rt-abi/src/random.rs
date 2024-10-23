
bitflags::bitflags! {
    pub struct GetRandomFlags: crate::bindings::get_random_flags {
        const NON_BLOCKING = crate::bindings::GET_RANDOM_NON_BLOCKING;
    }
}

pub fn twz_rt_get_random(buf: &mut [u8], flags: GetRandomFlags) -> usize {
    unsafe {
        crate::bindings::twz_rt_get_random(buf.as_mut_ptr().cast(), buf.len(), flags.bits())
    }
}
