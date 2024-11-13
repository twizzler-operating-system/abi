
pub type SystemInfo = crate::bindings::system_info;

impl SystemInfo {
    pub fn clock_monotonicity(&self) -> crate::time::Monotonicity {
        self.clock_monotonicity.into()
    }
}

pub fn twz_rt_get_sysinfo() -> SystemInfo {
    unsafe {
        crate::bindings::twz_rt_get_sysinfo()
    }
}
