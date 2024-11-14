#![allow(unused_variables)]
use core::time::Duration;

pub type ThreadId = crate::bindings::thread_id;
pub type TlsIndex = crate::bindings::tls_index;
pub type FutexWord = crate::bindings::futex_word;
pub type AtomicFutexWord = core::sync::atomic::AtomicU32;
pub type ThreadSpawnArgs = crate::bindings::spawn_args;

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum SpawnError {
    Other = crate::bindings::spawn_error_Spawn_Other,
    InvalidArgument = crate::bindings::spawn_error_Spawn_InvalidArgument,
    ObjectNotFound = crate::bindings::spawn_error_Spawn_ObjectNotFound,
    PermissionDenied = crate::bindings::spawn_error_Spawn_PermissionDenied,
    KernelError = crate::bindings::spawn_error_Spawn_KernelError,
}

impl From<SpawnError> for crate::bindings::spawn_error {
    fn from(value: SpawnError) -> Self {
        value as Self
    }
}

impl TryFrom<crate::bindings::spawn_error> for SpawnError {
    type Error = ();
    fn try_from(value: crate::bindings::spawn_error) -> Result<Self, ()> {
        Ok(match value {
            crate::bindings::spawn_error_Spawn_Other => SpawnError::Other,
            crate::bindings::spawn_error_Spawn_InvalidArgument => SpawnError::InvalidArgument,
            crate::bindings::spawn_error_Spawn_ObjectNotFound => SpawnError::ObjectNotFound,
            crate::bindings::spawn_error_Spawn_PermissionDenied => SpawnError::PermissionDenied,
            crate::bindings::spawn_error_Spawn_KernelError => SpawnError::KernelError,
            crate::bindings::spawn_error_Spawn_Success => return Err(()),
            _ => SpawnError::Other,
        })
    }
}

impl From<Result<ThreadId, SpawnError>> for crate::bindings::spawn_result {
    fn from(value: Result<ThreadId, SpawnError>) -> Self {
        match value {
            Ok(id) => Self {
                id,
                err: crate::bindings::spawn_error_Spawn_Success,
            },
            Err(e) => Self {
                id: 0,
                err: e.into(),
            }
        }
    }
}

impl Into<Result<ThreadId, SpawnError>> for crate::bindings::spawn_result {
    fn into(self) -> Result<ThreadId, SpawnError> {
        if let Ok(e) = self.err.try_into() {
            return Err(e);
        }
        Ok(self.id)
    }
}

#[repr(u32)]
pub enum JoinError {
    Other = crate::bindings::join_result_Join_Other,
    ThreadNotFound = crate::bindings::join_result_Join_ThreadNotFound,
    Timeout = crate::bindings::join_result_Join_Timeout,
}

impl From<JoinError> for crate::bindings::join_result {
    fn from(value: JoinError) -> Self {
        value as Self
    }
}

pub fn twz_rt_futex_wait(word: &AtomicFutexWord, expected: FutexWord, timeout: Option<Duration>) -> bool {
    unsafe {
        crate::bindings::twz_rt_futex_wait(word.as_ptr().cast(), expected, timeout.into())
    }
}

pub fn twz_rt_futex_wake(word: &AtomicFutexWord, max: Option<usize>) -> bool {
    let max = match max {
        Some(max) => max as i64,
        None => crate::bindings::FUTEX_WAKE_ALL,
    };
    unsafe {
        crate::bindings::twz_rt_futex_wake(word.as_ptr().cast(), max)
    }
}

pub fn twz_rt_yield() {
    unsafe {
        crate::bindings::twz_rt_yield_now();
    }
}

pub fn twz_rt_sleep(dur: Duration) {
    unsafe {
        crate::bindings::twz_rt_sleep(dur.into());
    }
}

pub fn twz_rt_set_thread_name(name: &core::ffi::CStr) {
    unsafe {
        crate::bindings::twz_rt_set_name(name.as_ptr());
    }
}

pub fn twz_rt_tls_get_addr(index: &TlsIndex) {
    unsafe {
        crate::bindings::twz_rt_tls_get_addr(index as *const _ as *mut _);
    }
}

pub fn twz_rt_spawn_thread(args: ThreadSpawnArgs) -> Result<ThreadId, SpawnError> {
    unsafe {
        crate::bindings::twz_rt_spawn_thread(args).into()
    }
}

pub fn twz_rt_join_thread(id: ThreadId, timeout: Option<Duration>) -> Result<(), JoinError> {
    unsafe {
        match crate::bindings::twz_rt_join_thread(id, timeout.into()) {
            crate::bindings::join_result_Join_Success => Ok(()),
            crate::bindings::join_result_Join_Timeout => Err(JoinError::Timeout),
            crate::bindings::join_result_Join_ThreadNotFound => Err(JoinError::ThreadNotFound),
            _ => Err(JoinError::Other),
        }
    }
}
