//! Low-level runtime functionality.

/// Type for exit code.
pub type ExitCode = crate::bindings::exit_code;

/// Exit with the provided error code. If the main thread for a program
/// exits, the remaining threads will exit as well.
#[cfg(not(feature = "kernel"))]
pub fn twz_rt_exit(code: ExitCode) -> ! {
    unsafe {
        crate::bindings::twz_rt_exit(code);
        unreachable!()
    }
}

/// Abort execution due to unrecoverable language error.
#[cfg(not(feature = "kernel"))]
pub fn twz_rt_abort() -> ! {
    unsafe {
        crate::bindings::twz_rt_abort();
        unreachable!()
    }
}

/// Call this before calling main, after initializing the runtime.
/// If this function returns None, then call main. Otherwise, act
/// as if main returned the provided [ExitCode].
#[cfg(not(feature = "kernel"))]
pub fn twz_rt_pre_main_hook() -> Option<ExitCode> {
    unsafe { crate::bindings::twz_rt_pre_main_hook().into() }
}

impl From<crate::bindings::option_exit_code> for Option<ExitCode> {
    #[inline]
    fn from(value: crate::bindings::option_exit_code) -> Self {
        if value.is_some == 0 {
            None
        } else {
            Some(value.value)
        }
    }
}

/// Call this after return from main, before running destructors.
#[cfg(not(feature = "kernel"))]
pub fn twz_rt_post_main_hook() {
    unsafe {
        crate::bindings::twz_rt_post_main_hook();
    }
}

/// Called by security context code on compartment entry
#[cfg(not(feature = "kernel"))]
pub fn twz_rt_cross_compartment_entry() {
    unsafe {
        crate::bindings::twz_rt_cross_compartment_entry();
    }
}

pub use crate::bindings::{
    basic_aux as BasicAux, basic_return as BasicReturn, comp_init_info as CompartmentInitInfo,
    ctor_set as CtorSet, init_info_ptrs as InitInfoPtrs, minimal_init_info as MinimalInitInfo,
    runtime_info as RuntimeInfo, RUNTIME_INIT_COMP, RUNTIME_INIT_MIN, RUNTIME_INIT_MONITOR,
};

// Safety: this holds functions pointers, but these pointers have 'static lifetime.
unsafe impl Send for CtorSet {}

// Safety: this holds functions pointers, but these pointers have 'static lifetime.
unsafe impl Sync for CtorSet {}

/// The entry point for the runtime. Not for public use.
#[cfg(not(feature = "kernel"))]
pub fn twz_rt_runtime_entry(
    info: *const RuntimeInfo,
    std_entry: unsafe extern "C-unwind" fn(BasicAux) -> BasicReturn,
) -> ! {
    unsafe {
        crate::bindings::twz_rt_runtime_entry(info, Some(std_entry));
        unreachable!()
    }
}

#[cfg(all(feature = "rt0", not(feature = "kernel")))]
pub mod rt0 {
    //! rt0 defines a collection of functions that the basic Rust ABI expects to be defined by some
    //! part of the C runtime:
    //!
    //!   - __tls_get_addr for handling non-local TLS regions.
    //!   - _start, the entry point of an executable (per-arch, as this is assembly code).

    #[cfg(target_arch = "aarch64")]
    #[no_mangle]
    #[naked]
    pub unsafe extern "C" fn _start() {
        core::arch::naked_asm!(
            "b {entry}",
            entry = sym entry,
        );
    }

    #[cfg(target_arch = "x86_64")]
    #[no_mangle]
    #[naked]
    pub unsafe extern "C" fn _start() {
        // Align the stack and jump to rust code. If we come back, trigger an exception.
        core::arch::naked_asm!(
            "and rsp, 0xfffffffffffffff0",
            "call {entry}",
            "ud2",
            entry = sym entry,
        );
    }

    #[used]
    // Ensure the compiler doesn't optimize us away!
    static ENTRY: unsafe extern "C" fn() = _start;

    use super::{BasicAux, BasicReturn, RuntimeInfo};

    // The C-based entry point coming from arch-specific assembly _start function.
    unsafe extern "C" fn entry(arg: usize) -> ! {
        // Just trampoline to rust-abi code.
        rust_entry(arg as *const _)
    }

    /// Entry point for Rust code wishing to start from rt0.
    ///
    /// # Safety
    /// Do not call this unless you are bootstrapping a runtime.
    pub unsafe fn rust_entry(arg: *const RuntimeInfo) -> ! {
        // All we need to do is grab the runtime and call its init function. We want to
        // do as little as possible here.
        super::twz_rt_runtime_entry(arg, std_entry_from_runtime)
    }

    extern "C-unwind" {
        fn std_entry_from_runtime(aux: BasicAux) -> BasicReturn;
    }
}
