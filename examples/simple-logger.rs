use log::{debug, error, info, trace, warn, LevelFilter};
use oslog::OSLogger;

fn main() {
    OSLogger::new("com.example.test")
        .with_level(LevelFilter::Debug)
        .with_category("Settings", LevelFilter::Trace)
        .init()
        .unwrap();

    // Maps to OS_LOG_TYPE_DEBUG
    trace!(target: "Settings", "This is a Debug message sent to the Settings category");

    // Maps to OS_LOG_TYPE_INFO
    debug!("This is an Info message with no category");

    // Maps to OS_LOG_TYPE_DEFAULT
    info!(target: "Parsing", "This is an Default message sent to the Parsing category which is created on the fly");

    // Maps to OS_LOG_TYPE_ERROR
    warn!("This is an Error message with no category");

    // Maps to OS_LOG_TYPE_FAULT
    error!("This is a Fault message with no category");
}
