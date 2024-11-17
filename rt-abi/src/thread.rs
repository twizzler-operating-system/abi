//! Runtime interface for threads.

#![allow(unused_variables)]
use core::time::Duration;

/// Runtime-internal thread ID.
pub type ThreadId = crate::bindings::thread_id;
/// Index of a TLS variable.
pub type TlsIndex = crate::bindings::tls_index;
/// Type of a linux-like wait point.
pub type FutexWord = crate::bindings::futex_word;
/// Atomic futex word, for a linux-like thread wait.
pub type AtomicFutexWord = core::sync::atomic::AtomicU32;
/// Arguments to spawn.
pub type ThreadSpawnArgs = crate::bindings::spawn_args;

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
/// Possible spawn error types.
pub enum SpawnError {
    /// Unclassified error.
    Other = crate::bindings::spawn_error_Spawn_Other,
    /// Argument was invalid.
    InvalidArgument = crate::bindings::spawn_error_Spawn_InvalidArgument,
    /// A specified object was not found.
    ObjectNotFound = crate::bindings::spawn_error_Spawn_ObjectNotFound,
    /// Permission was denied.
    PermissionDenied = crate::bindings::spawn_error_Spawn_PermissionDenied,
    /// The kernel encountered an error when spawning the thread.
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
            },
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
/// Possible join error states.
pub enum JoinError {
    /// Unclassified error.
    Other = crate::bindings::join_result_Join_Other,
    /// The specified thread was not found.
    ThreadNotFound = crate::bindings::join_result_Join_ThreadNotFound,
    /// The operation timed out.
    Timeout = crate::bindings::join_result_Join_Timeout,
}

impl From<JoinError> for crate::bindings::join_result {
    fn from(value: JoinError) -> Self {
        value as Self
    }
}

/// If the futex word pointed to by `word` is equal to expected, put the thread to sleep. This
/// operation is atomic -- the thread is enqueued on the sleep queue _first_, before the equality
/// check. Returns false on timeout, true on all other cases.
pub fn twz_rt_futex_wait(
    word: &AtomicFutexWord,
    expected: FutexWord,
    timeout: Option<Duration>,
) -> bool {
    unsafe { crate::bindings::twz_rt_futex_wait(word.as_ptr().cast(), expected, timeout.into()) }
}

/// Wake up up to max threads waiting on `word`. If max is None, wake up all threads.
pub fn twz_rt_futex_wake(word: &AtomicFutexWord, max: Option<usize>) -> bool {
    let max = match max {
        Some(max) => max as i64,
        None => crate::bindings::FUTEX_WAKE_ALL,
    };
    unsafe { crate::bindings::twz_rt_futex_wake(word.as_ptr().cast(), max) }
}

/// Yield the calling thread.
pub fn twz_rt_yield() {
    unsafe {
        crate::bindings::twz_rt_yield_now();
    }
}

/// Sleep the calling thread for duration `dur`.
pub fn twz_rt_sleep(dur: Duration) {
    unsafe {
        crate::bindings::twz_rt_sleep(dur.into());
    }
}

/// Set the name of the calling thread.
pub fn twz_rt_set_thread_name(name: &core::ffi::CStr) {
    unsafe {
        crate::bindings::twz_rt_set_name(name.as_ptr());
    }
}

/// Get the address of a given TLS variable.
pub fn twz_rt_tls_get_addr(index: &TlsIndex) -> *mut u8 {
    unsafe { crate::bindings::twz_rt_tls_get_addr(index as *const _ as *mut _).cast() }
}

/// Spawn a thread. On success, that thread starts executing concurrently with the return of this
/// function.
pub fn twz_rt_spawn_thread(args: ThreadSpawnArgs) -> Result<ThreadId, SpawnError> {
    unsafe { crate::bindings::twz_rt_spawn_thread(args).into() }
}

/// Wait for a thread to exit, optionally timing out.
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
