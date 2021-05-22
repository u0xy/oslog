use log::{debug, error, info, trace, warn, LevelFilter};
use oslog::OSLogger;

fn main() {
    OSLogger::new("com.example.test")
        .with_level(LevelFilter::Debug)
        .with_category("Settings", LevelFilter::Trace)
        .init()
        .unwrap();

    // Maps to OS_LOG_TYPE_DEBUG
    trace!(target: "Settings", "This is a Trace message");

    // Maps to OS_LOG_TYPE_INFO
    debug!("This is a Debug message");

    // Maps to OS_LOG_TYPE_DEFAULT
    info!(target: "Parsing", "This is an Info message");

    // Maps to OS_LOG_TYPE_ERROR
    warn!("This is a Warn message");

    // Maps to OS_LOG_TYPE_FAULT
    error!("This is an Error message");
}
