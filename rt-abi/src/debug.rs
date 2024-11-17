//! Functions for interfacing with debug info from the runtime.

use core::mem::MaybeUninit;

use crate::object::ObjectHandle;

/// Information about loaded image program headers.
pub type DlPhdrInfo = crate::bindings::dl_phdr_info;
/// Loaded image identifier.
pub type LoadedImageId = crate::bindings::loaded_image_id;

/// A loaded runtime program component, and the associated image (executable or library) file.
/// Contains debugging info and program header info.
#[repr(transparent)]
pub struct LoadedImage(crate::bindings::loaded_image);

/// The ID of the root (executable, probably) loaded image.
pub use crate::bindings::TWZ_RT_EXEID;

impl LoadedImage {
    /// Get a byte slice of the image.
    pub fn image(&self) -> &[u8] {
        // Safety: the runtime ensures that these are valid for a byte slice.
        unsafe { core::slice::from_raw_parts(self.0.image_start.cast(), self.0.image_len) }
    }

    /// Get an owned object handle for the image.
    pub fn handle(&self) -> ObjectHandle {
        // Since the internal object handle is from the C ffi, we need
        // to manually manage the refcounts. The phantom handle represents
        // our handle (self.0.handle), so we will forget it to ensure its
        // drop impl doesn't run.
        let phantom_handle = ObjectHandle(self.0.image_handle);
        // This handle is the one we are handing out.
        let handle = phantom_handle.clone();
        core::mem::forget(phantom_handle);
        handle
    }

    /// Get the runtime ID of the loaded image.
    pub fn id(&self) -> LoadedImageId {
        self.0.id
    }

    /// Get the [DlPhdrInfo] for this loaded image.
    pub fn dl_info(&self) -> &DlPhdrInfo {
        &self.0.dl_info
    }
}

impl Clone for LoadedImage {
    fn clone(&self) -> Self {
        // See LoadedImage::handle above for an explanation.
        let phantom_handle = ObjectHandle(self.0.image_handle);
        let handle = phantom_handle.clone();
        core::mem::forget(phantom_handle);
        // This time, we forget the handle so its drop doesn't run,
        // since that will be our cloned handle.
        core::mem::forget(handle);
        Self(self.0)
    }
}

impl Drop for LoadedImage {
    fn drop(&mut self) {
        let _handle = ObjectHandle(self.0.image_handle);
        // Drop the object handle, since it's stored
        // as a C type.
    }
}

/// Return the [LoadedImage] associated with the given [LoadedImageId] in the runtime.
/// If no such ID exists, return None.
pub fn twz_rt_get_loaded_image(id: LoadedImageId) -> Option<LoadedImage> {
    unsafe {
        let mut li = MaybeUninit::uninit();
        if !crate::bindings::twz_rt_get_loaded_image(id, li.as_mut_ptr()) {
            return None;
        }

        // Safety: the call above returning true ensures the value is initialized.
        Some(LoadedImage(li.assume_init()))
    }
}

/*
/// Iterate over the loaded program components known to the runtime. This function has similar semantics
/// to C's dl_iter_phdr. This function will call f with a [DlPhdrInfo] for each known loaded component,
/// until all components are processed, or until f returns a non-zero value (which this function then returns).
pub fn twz_rt_iter_phdr(f: &dyn Fn(&DlPhdrInfo) -> i32) -> i32 {
    // The twz_rt_iter_phdr call acts like dl_iter_phdr, so we'll need to trampoline to call f.
    extern "C-unwind" fn trampoline(info: *const DlPhdrInfo, size: usize, data: *mut core::ffi::c_void) -> i32 {
        unsafe {
            // Safety: the value of data is passed in by us below, and the contract with the runtime says that
            // the pointer to info is valid.
            let f: Box<&dyn Fn(&DlPhdrInfo) -> i32> = data.cast::<*mut Box<_>>().as_mut().unwrap();
            f(&*info)
        }
    }

    let mut data = Box::new(f);
    unsafe { crate::bindings::twz_rt_iter_phdr(Some(trampoline), &mut data as *mut _) }
}
*/
