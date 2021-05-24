use oslog::{cstr, OSLog};

fn main() {
    let log_settings = OSLog::new("com.example.test", "Settings");

    // Uses OS_LOG_TYPE_DEBUG
    log_settings.debug(cstr!("This is a Debug message"));

    // Uses OS_LOG_TYPE_INFO
    log_settings.info(cstr!("This is a Info message"));

    // Uses OS_LOG_TYPE_DEFAULT
    log_settings.default(cstr!("This is an Default message"));

    // Uses OS_LOG_TYPE_ERROR
    log_settings.error(cstr!("This is an Error message"));

    // Uses OS_LOG_TYPE_FAULT
    log_settings.fault(cstr!("This is a Fault message"));
}
