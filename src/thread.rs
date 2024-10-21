use core::num::NonZeroUsize;

pub fn available_parallelism() -> NonZeroUsize {
    let ap: usize = unsafe { crate::bindings::available_parallelism() }.try_into().unwrap_or(1);
    ap.try_into().unwrap_or(NonZeroUsize::new(1).unwrap())
}
