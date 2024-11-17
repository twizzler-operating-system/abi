#![no_std]

pub type ObjID = bindings::rt_objid;

#[allow(
    non_camel_case_types,
    dead_code,
    non_upper_case_globals,
    improper_ctypes
)]
pub(crate) mod bindings;
