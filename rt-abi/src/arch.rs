#[cfg(target_arch = "x86_64")]
#[path = "arch/x86_64.rs"]
mod imp;

#[cfg(target_arch = "aarch64")]
#[path = "arch/aarch64.rs"]
mod imp;

pub use imp::*;
