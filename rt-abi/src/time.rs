use core::time::Duration;

#[repr(u32)]
pub enum Monotonicity {
    NonMonotonic = crate::bindings::monotonicity_NonMonotonic,
    Weak = crate::bindings::monotonicity_WeakMonotonic,
    Strict = crate::bindings::monotonicity_StrongMonotonic,
}

impl Into<u32> for Monotonicity {
    fn into(self) -> u32 {
        self as u32
    }
}

pub fn twz_rt_get_monotonic_time() -> Duration {
    todo!()
}

pub fn twz_rt_get_system_time() -> Duration {
    todo!()
}

impl From<Duration> for crate::bindings::duration {
    fn from(value: Duration) -> Self {
        Self {
            seconds: value.as_secs(),
            nanos: value.subsec_nanos(),
        }
    }
}

impl From<crate::bindings::duration> for Duration {
    fn from(value: crate::bindings::duration) -> Self {
        Self::new(value.seconds, value.nanos)
    }
}
