#![allow(unused_variables)]
use core::time::Duration;

pub type ThreadId = crate::bindings::thread_id;
pub type TlsIndex = crate::bindings::tls_index;
pub type FutexWord = crate::bindings::futex_word;
pub type AtomicFutexWord = core::sync::atomic::AtomicU32;
pub type ThreadSpawnArgs = crate::bindings::spawn_args;

#[repr(u32)]
pub enum SpawnError {
    Other = crate::bindings::spawn_error_Spawn_Other,
    InvalidArgument = crate::bindings::spawn_error_Spawn_InvalidArgument,
    ObjectNotFound = crate::bindings::spawn_error_Spawn_ObjectNotFound,
    PermissionDenied = crate::bindings::spawn_error_Spawn_PermissionDenied,
    KernelError = crate::bindings::spawn_error_Spawn_KernelError,
}

#[repr(u32)]
pub enum JoinError {
    ThreadNotFound = crate::bindings::join_result_Join_ThreadNotFound,
    Timeout = crate::bindings::join_result_Join_Timeout,
}

pub fn twz_rt_futex_wait(word: &AtomicFutexWord, expected: FutexWord, timeout: Option<Duration>) -> bool {
    todo!()
}

pub fn twz_rt_futex_wake(word: &AtomicFutexWord, max: Option<usize>) -> bool {
    todo!()
}

pub fn twz_rt_yield() {
    todo!()
}

pub fn twz_rt_sleep(dur: Duration) {
    todo!()
}

pub fn twz_rt_set_name(name: &core::ffi::CStr) {
    todo!()
}

pub fn twz_rt_tls_get_addr(index: &TlsIndex) {
    todo!()
}

pub fn twz_rt_spawn_thread(args: ThreadSpawnArgs) -> Result<ThreadId, SpawnError> {
    todo!()
}

pub fn twz_rt_join_thread(id: ThreadId) -> Result<(), JoinError> {
    todo!()
}
