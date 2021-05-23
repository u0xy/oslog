//! A minimal wrapper around Apple's [Unified Logging System]. On macOS, this
//! allows you to capture telemetry from your Rust programs for debugging and
//! performance analysis.
//!
//!
//! ## Overview
//!
//! This crate provides two facilities:
//!
//! - the first one focuses on compatibility with the [log] crate and
//! thus provides only message logging to category loggers (named "targets" in
//! the [log] crate).
//! - the second one models as closely as it can the [Swift/ObjC OSLog API]:
//! this includes
//!   - usual message logging to category loggers,
//!   - activity tracing across categories
//!   - and [performance logging with signposts] for very detailed profiling
//!   with [Xcode Instruments].
//!
//! To help you choose a facility, let's briefly explain logging features and
//! tools on macOS.
//!
//! 1. Both facilities can be used to make your program send log messages to
//!    the macOS Logging system. Note that, because macOS has deprecated
//!    sending logs to `/var/log` some time ago, in order to search and view
//!    the messages, you can use the [log command line tool] or the [Console
//!    App] or one its [alternatives]. Besides the Apple official docs, this
//!    article, although somewhat dated, provides a [good overview of the
//!    Unified Logging System].
//!
//! ![Console App Window](https://raw.githubusercontent.com/u0xy/oslog/screenshots/screenshots/console-app.png)
//!
//! 2. The `OSLog` facility also allows you to trace [Activities] across
//!    categories. You can think of this as a semantic layer on top of
//!    potentially many concurrent messages logged.
//!
//! ![Console App Activities](https://raw.githubusercontent.com/u0xy/oslog/screenshots/screenshots/console-app-activities.png)
//!
//! 3. For performance analysis, the `OSLog` facility helps you instrument
//!    your code for profiling with "signpost" markers. Then you profile your
//!    program by running it in a specific run environment: on macOS, this
//!    environment is provided by the [`xctrace`] command line tool which
//!    generates a trace bundle. In practice, you run:
//!
//! ```sh
//! xcrun xctrace --template "Time Profiler" \
//!     --launch -- ./target/release/examples/simple-signposting
//! ```
//!
//! To make this even simpler, the Rust ecosystem provides the
//! [cargo-instruments] crate to do exactly this, with a nice Cargo
//! integration (and is available via `brew install cargo-instruments`):
//!
//! ```sh
//! cargo instruments -t "Time Profiler" --example simple-signposting
//! ```
//!
//! The trace bundle should be opened with [Xcode Instruments]
//!
//! ![Instruments Systrace Profiler](https://raw.githubusercontent.com/u0xy/oslog/screenshots/screenshots/instruments-system-trace.png)
//!
//! With this in mind, you can choose which of the facilities you want to use.
//!
//!
//! ## Using `oslog::OSLogger` for `log` crate support
//!
//! Because it depends on the `log` crate, this logger is available with the
//! `"logger"` feature.
//!
//! As a reminder, on macOS, the logging levels are `Debug`, `Info`,
//! `Default`, `Error`, and `Fault`. They map to the level of the [log] crate
//! as follows
//!
//! | OSLog level | log crate level |
//! | ---         | ---             |
//! | Debug       | Trace           |
//! | Info        | Debug           |
//! | Default     | Info            |
//! | Error       | Warn            |
//! | Fault       | Error           |
//!
//! Here is a full example.
//!
//! ```rust
//! OSLogger::new("com.example.test")
//!     .with_level(LevelFilter::Debug)
//!     .with_category("Settings", LevelFilter::Trace)
//!     .with_category("Parsing", LevelFilter::Info)
//!     .init()
//!     .unwrap();
//!
//! trace!(target: "Settings", "Debug message to the `Settings` category");
//! debug!("Info message with no category");
//! info!(target: "Parsing", "Default message to the `Parsing` category");
//! warn!("Error message with no category");
//! error!("Fault message with no category");
//! ```
//!
//! Each category logger is allocated and stored in a map for the lifetime of
//! the program. If you care about not allocating during logging (as you
//! should), make sure
//!
//! - you have created all category loggers beforehand, as shown above,
//! - and make sure you pass `&CStr` arguments, instead of `&str`.
//!
//! Otherwise, if you log to a non-existing "target", the corresponding
//! category logger will be allocated once before logging. The allocations are
//! extremely small, but still. In the same vein, if you pass `&str` messages
//! instead of `&CStr`, a null-terminating `CString` will have to be created
//! for the FFI call to the C functions.
//!
//! When making use of targets (`info!(target: "t", "m");`), you should be
//! aware that a new log is allocated and stored in a map for the lifetime of
//! the program.  I expect log allocations are extremely small, but haven't
//! attempted to verify it.
//!
//!
//! ## Using `oslog::OSLog` for logging and profiling
//!
//! Logging Quickstart
//!
//! ```
//! use oslog::{OSLog, cstr}
//!
//! let log_settings = OSLog::new("com.example.test", "Settings");
//! //                             subsystem ~~^   category ~^
//!
//! log_settings.debug("This is a Debug message");
//! log_settings.info("This is a Info message");
//! log_settings.default("This is an Default message");
//! log_settings.error("This is an Error message");
//! log_settings.fault("This is a Fault message");
//! ```
//!
//!
//! ## Performance analysis
//!
//! For performance analysis, you need to profile your program using [Xcode
//! Instruments]. This can be done quite easily with the [cargo-instruments]
//! crate.
//!
//!
//! # Missing features
//!
//! * Activities
//! * Tracing
//! * Native support for line numbers and file names.
//!
//! [Unified Logging System]: https://developer.apple.com/documentation/os/logging
//! [Swift/ObjC OSLog API]: https://developer.apple.com/documentation/os/logging
//! [Console App]: https://support.apple.com/guide/console/welcome/mac
//! [alternatives]: https://eclecticlight.co/consolation-t2m2-and-log-utilities/
//! [Activities]: https://developer.apple.com/documentation/os/logging/collecting_log_messages_in_activities
//! [log command line tool]: https://developer.apple.com/documentation/os/logging/viewing_log_messages
//! [log]: https://docs.rs/log
//! [cargo-instruments]: https://crates.io/crates/cargo-instruments
//! [performance logging with signposts]: https://developer.apple.com/videos/play/wwdc2018/405/
//! [Xcode Instruments]: https://developer.apple.com/library/archive/documentation/ToolsLanguages/Conceptual/Xcode_Overview/MeasuringPerformance.html
//! [`xctrace`]: https://developer.apple.com/documentation/xcode-release-notes/xcode-12-release-notes
//! [good overview of the Unified Logging System]: https://eclecticlight.co/2018/03/19/macos-unified-log-1-why-what-and-how/
//!

// #![warn(missing_docs)]
// #![warn(missing_doc_code_examples)]

mod sys;

#[cfg(feature = "logger")]
mod logger;

#[cfg(feature = "logger")]
pub use logger::OSLogger;

#[cfg(feature = "signpost")]
mod signpost;

#[cfg(feature = "signpost")]
pub use signpost::OSSignpostID;

use std::ffi::{c_void, CString};

// Re-exports the `cstr!` macro for convenience
pub use cstr::cstr;

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

pub struct OSLog {
    inner: sys::os_log_t,
}

unsafe impl Send for OSLog {}
unsafe impl Sync for OSLog {}

impl Drop for OSLog {
    fn drop(&mut self) {
        unsafe {
            if self.inner != sys::wrapped_get_default_log() {
                sys::os_release(self.inner as *mut c_void);
            }
        }
    }
}

impl OSLog {
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

    /// Returns a Boolean value that indicates whether the log object has the
    /// specified logging level enabled.
    pub fn level_is_enabled(&self, level: Level) -> bool {
        unsafe { sys::os_log_type_enabled(self.inner, level as u8) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subsystem_interior_null() {
        let log = OSLog::new("com.example.oslog\0test", "category");
        log.with_level(Level::Debug, "Hi");
    }

    #[test]
    fn test_category_interior_null() {
        let log = OSLog::new("com.example.oslog", "category\0test");
        log.with_level(Level::Debug, "Hi");
    }

    #[test]
    fn test_message_interior_null() {
        let log = OSLog::new("com.example.oslog", "category");
        log.with_level(Level::Debug, "Hi\0test");
    }

    #[test]
    fn test_message_emoji() {
        let log = OSLog::new("com.example.oslog", "category");
        log.with_level(Level::Debug, "\u{1F601}");
    }

    #[test]
    fn test_global_log_with_level() {
        let log = OSLog::global();
        log.with_level(Level::Debug, "Debug");
        log.with_level(Level::Info, "Info");
        log.with_level(Level::Default, "Default");
        log.with_level(Level::Error, "Error");
        log.with_level(Level::Fault, "Fault");
    }

    #[test]
    fn test_global_log() {
        let log = OSLog::global();
        log.debug("Debug");
        log.info("Info");
        log.default("Default");
        log.error("Error");
        log.fault("Fault");
    }

    #[test]
    fn test_custom_log_with_level() {
        let log = OSLog::new("com.example.oslog", "testing");
        log.with_level(Level::Debug, "Debug");
        log.with_level(Level::Info, "Info");
        log.with_level(Level::Default, "Default");
        log.with_level(Level::Error, "Error");
        log.with_level(Level::Fault, "Fault");
    }

    #[test]
    fn test_custom_log() {
        let log = OSLog::new("com.example.oslog", "testing");
        log.debug("Debug");
        log.info("Info");
        log.default("Default");
        log.error("Error");
        log.fault("Fault");
    }
}
