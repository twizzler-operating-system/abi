#![no_std]

pub mod alloc;
pub mod core;
pub mod thread;
pub mod object;
pub mod fd;
pub mod io;
pub mod time;
pub mod debug;
pub mod info;
pub mod random;

#[allow(non_camel_case_types, dead_code, non_upper_case_globals, improper_ctypes)]
pub(crate) mod bindings;

