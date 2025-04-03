//! Runtime interface for threads.

#![allow(unused_variables)]
use core::time::Duration;

use crate::Result;

/// Runtime-internal thread ID.
pub type ThreadId = crate::bindings::thread_id;
/// Index of a TLS variable.
pub type TlsIndex = crate::bindings::tls_index;
/// TLS desc
pub type TlsDesc = crate::bindings::tls_desc;
/// Type of a linux-like wait point.
pub type FutexWord = crate::bindings::futex_word;
/// Atomic futex word, for a linux-like thread wait.
pub type AtomicFutexWord = core::sync::atomic::AtomicU32;
/// Arguments to spawn.
pub type ThreadSpawnArgs = crate::bindings::spawn_args;

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
pub fn twz_rt_spawn_thread(args: ThreadSpawnArgs) -> Result<ThreadId> {
    unsafe { crate::bindings::twz_rt_spawn_thread(args).into() }
}

/// Wait for a thread to exit, optionally timing out.
pub fn twz_rt_join_thread(id: ThreadId, timeout: Option<Duration>) -> Result<()> {
    unsafe { crate::bindings::twz_rt_join_thread(id, timeout.into()).into() }
}
