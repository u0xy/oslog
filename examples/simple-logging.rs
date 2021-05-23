use oslog::OSLog;

fn main() {
    let log_settings = OSLog::new("com.example.test", "Settings");

    // Uses OS_LOG_TYPE_DEBUG
    log_settings.debug("This is a Debug message");

    // Uses OS_LOG_TYPE_INFO
    log_settings.info("This is a Info message");

    // Uses OS_LOG_TYPE_DEFAULT
    log_settings.default("This is an Default message");

    // Uses OS_LOG_TYPE_ERROR
    log_settings.error("This is an Error message");

    // Uses OS_LOG_TYPE_FAULT
    log_settings.fault("This is a Fault message");
}
