
pub type SystemInfo = crate::bindings::system_info;

impl SystemInfo {
    pub fn clock_monotonicity(&self) -> crate::time::Monotonicity {
        todo!()
    }
}

pub fn twz_rt_get_sysinfo() -> SystemInfo {
    todo!()
}
