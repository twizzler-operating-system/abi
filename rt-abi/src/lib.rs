#![no_std]
#![feature(allocator_api)]
#![cfg_attr(
    all(feature = "stderr", not(feature = "rustc-dep-of-std")),
    feature(io_error_inprogress)
)]
#![cfg_attr(
    all(feature = "stderr", not(feature = "rustc-dep-of-std")),
    feature(io_error_more)
)]

pub mod core;
#[allow(unused_imports)]
pub mod object;

pub mod alloc;
pub mod arch;
pub mod debug;
pub mod exec;
pub mod fd;
pub mod info;
pub mod io;
pub mod random;
pub mod thread;
pub mod time;

#[allow(
    non_camel_case_types,
    dead_code,
    non_upper_case_globals,
    improper_ctypes
)]
pub mod bindings;

pub mod error;

pub type Result<T> = ::core::result::Result<T, error::TwzError>;
