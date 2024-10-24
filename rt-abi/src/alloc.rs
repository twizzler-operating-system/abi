#![allow(unused_variables)]

use core::alloc::Layout;

bitflags::bitflags! {
    pub struct AllocFlags : crate::bindings::alloc_flags {
        const ZERO_MEMORY = crate::bindings::ZERO_MEMORY;
    }
}

pub fn twz_rt_malloc(layout: Layout, flags: AllocFlags) -> Option<*mut u8> {
    todo!()
}

pub fn twz_rt_realloc(ptr: *mut u8, layout: Layout, new_size: usize, flags: AllocFlags) -> Option<*mut u8> {
    todo!()   
}

pub fn twz_rt_dealloc(ptr: *mut u8, layout: Layout, flags: AllocFlags) {
    todo!()
}
