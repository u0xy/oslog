use crate::sys;
use crate::OSLog;
use std::ffi::{c_void, CStr};

#[cfg(feature = "signpost")]
impl OSLog {
    /// Marks an event in your code using a signpost.
    ///
    /// This calls [`os_signpost_event_emit()`] via FFI.
    ///
    /// [`os_signpost_event_emit()`]: https://developer.apple.com/documentation/os/os_signpost_event_emit?language=objc.
    pub fn signpost_event(&self, spid: &OSSignpostID, name: &CStr, format: &CStr, message: &CStr) {
        unsafe {
            // sys::wrapped_os_signpost_event_emit(
            //     self.inner,
            //     spid.inner,
            //     name.as_ptr(),
            //     format.as_ptr(),
            //     message.as_ptr(),
            // )
            sys::va_os_signpost_event_emit_with_type(
                self.inner,
                sys::OS_SIGNPOST_EVENT,
                spid.inner,
                name.as_ptr(),
                format.as_ptr(),
                message.as_ptr(),
            )
        }
    }
}

pub struct OSSignpostID {
    inner: sys::os_signpost_id_t,
}

impl Default for OSSignpostID {
    /// Creates a signpost identifier that's not unique but is cheap to
    /// create.
    ///
    /// If only one interval with a given os_log_t and interval name will ever
    /// be in flight at a time, or if you don't need to distinguish between
    /// signposts that overlap in time, use this convenience value. This can
    /// avoid having to share state between begin and end callsites and is
    /// very cheap to create.
    fn default() -> Self {
        OSSignpostID {
            inner: sys::OS_SIGNPOST_ID_EXCLUSIVE,
        }
    }
}
impl OSSignpostID {
    /// Creates a signpost identifier that's unique among signposts logged to
    /// a specified log.
    pub fn generate(log: &OSLog) -> OSSignpostID {
        OSSignpostID {
            inner: unsafe { sys::os_signpost_id_generate(log.inner) },
        }
    }

    /// Creates a signpost identifier that's unique among signposts logging to
    /// the specified log, using a pointer value to generate the unique value.
    ///
    /// Note: don't use this function if the activity needs to cross process
    /// boundaries.
    pub fn generate_with_pointer<T>(log: &OSLog, object: T) -> OSSignpostID
    where
        T: ptrplus::AsPtr,
    {
        let ptr = object.as_ptr() as *const c_void;
        OSSignpostID {
            inner: unsafe { sys::os_signpost_id_make_with_pointer(log.inner, ptr) },
        }
    }
}

unsafe impl Send for OSSignpostID {}
unsafe impl Sync for OSSignpostID {}

#[cfg(test)]
mod tests {
    use super::*;
    // Use the re-exported cstr! macro.
    use crate::cstr;

    #[test]
    /// If you were to profile this code run with `xcrun xctrace --template
    /// SomeTemplateWithOSSignpost` you get the following table of results in
    /// Xcode instruments:
    ///
    /// | Process           | Subsystem      | Category     | Name                       | Signpost ID              | Message                       |
    /// | ---               | ---            | ---          | ---                        | ---                      | ---                           |
    /// | simple-signposter | com.signposter | the-category | the-default-signpost-name  | OS_SIGNPOST_ID_EXCLUSIVE | the-default-signpost-message  |
    /// | simple-signposter | com.signposter | the-category | the-default-signpost-name2 | OS_SIGNPOST_ID_EXCLUSIVE | the-default-signpost-message2 |
    /// | simple-signposter | com.signposter | the-category | the-signpost-name          | 0x01                     | the-default-signpost-message  |
    /// | simple-signposter | com.signposter | the-category | the-signpost-name2         | 0x01                     | the-default-signpost-message2 |
    /// | simple-signposter | com.signposter | the-category | the-ref-signpost-name      | 0x74f22b67ffaee5d0       | the-default-signpost-message  |
    /// | simple-signposter | com.signposter | the-category | the-ref-signpost-name2     | 0x74f22b67ffaee5d0       | the-default-signpost-message2 |
    fn test_signpost_event_with_various_id_sources() {
        let log = OSLog::new("com.signposter", "the-category");

        // Log 2 events using the default signpost id
        //
        let signpost_id = OSSignpostID::default();
        log.signpost_event(
            &signpost_id,
            cstr!("the-default-signpost-name"),
            cstr!("%{public}s"),
            cstr!("the-default-signpost-message"),
        );
        log.signpost_event(
            &signpost_id,
            cstr!("the-default-signpost-name2"),
            cstr!("%{public}s"),
            cstr!("the-default-signpost-message2"),
        );

        // Log 2 events using a new random signpost id
        //
        let signpost_id = OSSignpostID::generate(&log);
        log.signpost_event(
            &signpost_id,
            cstr!("the-signpost-name"),
            cstr!("%{public}s"),
            cstr!("the-signpost-message"),
        );
        log.signpost_event(
            &signpost_id,
            cstr!("the-signpost-name2"),
            cstr!("%{public}s"),
            cstr!("the-signpost-message2"),
        );

        // Log 2 events using a signpost id generated from any pointer
        //
        let ref_object = String::from("reference-object");
        let signpost_id = OSSignpostID::generate_with_pointer(&log, &ref_object);
        log.signpost_event(
            &signpost_id,
            cstr!("the-ref-signpost-name"),
            cstr!("%{public}s"),
            cstr!("the-ref-signpost-message"),
        );
        log.signpost_event(
            &signpost_id,
            cstr!("the-ref-signpost-name2"),
            cstr!("%{public}s"),
            cstr!("the-ref-signpost-message2"),
        );
    }
}
