#![no_std]
#![feature(naked_functions)]
#![feature(allocator_api)]

#[cfg(all(feature = "std", not(feature = "kernel")))]
#[macro_use]
extern crate std;

pub mod core;
#[allow(unused_imports)]
pub mod object;

#[cfg(not(feature = "kernel"))]
pub mod alloc;
#[cfg(not(feature = "kernel"))]
pub mod arch;
#[cfg(not(feature = "kernel"))]
pub mod debug;
#[cfg(not(feature = "kernel"))]
pub mod fd;
#[cfg(not(feature = "kernel"))]
pub mod info;
#[cfg(not(feature = "kernel"))]
pub mod io;
#[cfg(not(feature = "kernel"))]
pub mod random;
#[cfg(not(feature = "kernel"))]
pub mod thread;
#[cfg(not(feature = "kernel"))]
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
