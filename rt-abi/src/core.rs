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

pub use crate::bindings::{
    basic_aux as BasicAux, basic_return as BasicReturn, comp_init_info as CompartmentInitInfo,
    init_info_ptrs as InitInfoPtrs, minimal_init_info as MinimalInitInfo,
    runtime_info as RuntimeInfo, RUNTIME_INIT_COMP, RUNTIME_INIT_MIN, RUNTIME_INIT_MONITOR,
};

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
