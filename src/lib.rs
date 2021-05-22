mod signpost;
mod sys;

#[cfg(feature = "logger")]
mod logger;

#[cfg(feature = "logger")]
pub use logger::OsLogger;

use std::ffi::{c_void, CStr, CString};

#[inline]
pub fn to_cstr(message: &str) -> CString {
    let fixed = message.replace('\0', "(null)");
    CString::new(fixed).unwrap()
}

#[repr(u8)]
pub enum Level {
    Debug = sys::OS_LOG_TYPE_DEBUG,
    Info = sys::OS_LOG_TYPE_INFO,
    Default = sys::OS_LOG_TYPE_DEFAULT,
    Error = sys::OS_LOG_TYPE_ERROR,
    Fault = sys::OS_LOG_TYPE_FAULT,
}

#[cfg(feature = "logger")]
impl From<log::Level> for Level {
    fn from(other: log::Level) -> Self {
        match other {
            log::Level::Trace => Self::Debug,
            log::Level::Debug => Self::Info,
            log::Level::Info => Self::Default,
            log::Level::Warn => Self::Error,
            log::Level::Error => Self::Fault,
        }
    }
}

pub struct OsLog {
    inner: sys::os_log_t,
}

unsafe impl Send for OsLog {}
unsafe impl Sync for OsLog {}

impl Drop for OsLog {
    fn drop(&mut self) {
        unsafe {
            if self.inner != sys::wrapped_get_default_log() {
                sys::os_release(self.inner as *mut c_void);
            }
        }
    }
}

impl OsLog {
    pub fn new(subsystem: &str, category: &str) -> Self {
        let subsystem = to_cstr(subsystem);
        let category = to_cstr(category);

        let inner = unsafe { sys::os_log_create(subsystem.as_ptr(), category.as_ptr()) };

        assert!(!inner.is_null(), "Unexpected null value from os_log_create");

        Self { inner }
    }

    pub fn global() -> Self {
        let inner = unsafe { sys::wrapped_get_default_log() };

        assert!(!inner.is_null(), "Unexpected null value for OS_DEFAULT_LOG");

        Self { inner }
    }

    pub fn with_level(&self, level: Level, message: &str) {
        let message = to_cstr(message);
        unsafe { sys::wrapped_os_log_with_type(self.inner, level as u8, message.as_ptr()) }
    }

    pub fn debug(&self, message: &str) {
        let message = to_cstr(message);
        unsafe { sys::wrapped_os_log_debug(self.inner, message.as_ptr()) }
    }

    pub fn info(&self, message: &str) {
        let message = to_cstr(message);
        unsafe { sys::wrapped_os_log_info(self.inner, message.as_ptr()) }
    }

    pub fn default(&self, message: &str) {
        let message = to_cstr(message);
        unsafe { sys::wrapped_os_log_default(self.inner, message.as_ptr()) }
    }

    pub fn error(&self, message: &str) {
        let message = to_cstr(message);
        unsafe { sys::wrapped_os_log_error(self.inner, message.as_ptr()) }
    }

    pub fn fault(&self, message: &str) {
        let message = to_cstr(message);
        unsafe { sys::wrapped_os_log_fault(self.inner, message.as_ptr()) }
    }

    pub fn level_is_enabled(&self, level: Level) -> bool {
        unsafe { sys::os_log_type_enabled(self.inner, level as u8) }
    }

    pub fn signpost_event(&self, spid: &OSSignpostID, name: &CStr, format: &CStr, message: &CStr) {
        unsafe {
            sys::wrapped_os_signpost_event_emit(
                self.inner,
                spid.inner,
                name.as_ptr(),
                format.as_ptr(),
                message.as_ptr(),
            )
        }
    }
}

//
// Signpost
//

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
    pub fn generate(log: &OsLog) -> OSSignpostID {
        OSSignpostID {
            inner: unsafe { sys::os_signpost_id_generate(log.inner) },
        }
    }

    /// Creates a signpost identifier that's unique among signposts logging to
    /// the specified log, using a pointer value to generate the unique value.
    ///
    /// Note: don't use this function if the activity needs to cross process
    /// boundaries.
    pub fn generate_with_pointer<T>(log: &OsLog, object: T) -> OSSignpostID
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
    use c_str_macro::c_str;

    #[test]
    fn test_subsystem_interior_null() {
        let log = OsLog::new("com.example.oslog\0test", "category");
        log.with_level(Level::Debug, "Hi");
    }

    #[test]
    fn test_category_interior_null() {
        let log = OsLog::new("com.example.oslog", "category\0test");
        log.with_level(Level::Debug, "Hi");
    }

    #[test]
    fn test_message_interior_null() {
        let log = OsLog::new("com.example.oslog", "category");
        log.with_level(Level::Debug, "Hi\0test");
    }

    #[test]
    fn test_message_emoji() {
        let log = OsLog::new("com.example.oslog", "category");
        log.with_level(Level::Debug, "\u{1F601}");
    }

    #[test]
    fn test_global_log_with_level() {
        let log = OsLog::global();
        log.with_level(Level::Debug, "Debug");
        log.with_level(Level::Info, "Info");
        log.with_level(Level::Default, "Default");
        log.with_level(Level::Error, "Error");
        log.with_level(Level::Fault, "Fault");
    }

    #[test]
    fn test_global_log() {
        let log = OsLog::global();
        log.debug("Debug");
        log.info("Info");
        log.default("Default");
        log.error("Error");
        log.fault("Fault");
    }

    #[test]
    fn test_custom_log_with_level() {
        let log = OsLog::new("com.example.oslog", "testing");
        log.with_level(Level::Debug, "Debug");
        log.with_level(Level::Info, "Info");
        log.with_level(Level::Default, "Default");
        log.with_level(Level::Error, "Error");
        log.with_level(Level::Fault, "Fault");
    }

    #[test]
    fn test_custom_log() {
        let log = OsLog::new("com.example.oslog", "testing");
        log.debug("Debug");
        log.info("Info");
        log.default("Default");
        log.error("Error");
        log.fault("Fault");
    }

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
        let log = OsLog::new("com.signposter", "the-category");

        let signpost_id = OSSignpostID::default();
        log.signpost_event(
            &signpost_id,
            c_str!("the-default-signpost-name"),
            c_str!("%{public}s"),
            c_str!("the-default-signpost-message"),
        );
        log.signpost_event(
            &signpost_id,
            c_str!("the-default-signpost-name2"),
            c_str!("%{public}s"),
            c_str!("the-default-signpost-message2"),
        );

        let signpost_id = OSSignpostID::generate(&log);
        log.signpost_event(
            &signpost_id,
            c_str!("the-signpost-name"),
            c_str!("%{public}s"),
            c_str!("the-signpost-message"),
        );
        log.signpost_event(
            &signpost_id,
            c_str!("the-signpost-name2"),
            c_str!("%{public}s"),
            c_str!("the-signpost-message2"),
        );

        let ref_object = String::from("reference-object");
        let signpost_id = OSSignpostID::generate_with_pointer(&log, &ref_object);
        log.signpost_event(
            &signpost_id,
            c_str!("the-ref-signpost-name"),
            c_str!("%{public}s"),
            c_str!("the-ref-signpost-message"),
        );
        log.signpost_event(
            &signpost_id,
            c_str!("the-ref-signpost-name2"),
            c_str!("%{public}s"),
            c_str!("the-ref-signpost-message2"),
        );
    }
}
