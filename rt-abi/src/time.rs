use core::time::Duration;

#[repr(u32)]
pub enum Monotonicity {
    NonMonotonic = crate::bindings::monotonicity_NonMonotonic,
    WeakMonotonic = crate::bindings::monotonicity_WeakMonotonic,
    StrongMonotonic = crate::bindings::monotonicity_StrongMonotonic,
}

pub fn twz_rt_get_monotonic_time() -> Duration {
    todo!()
}

pub fn twz_rt_get_system_time() -> Duration {
    todo!()
}
