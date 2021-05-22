A minimal wrapper around Apple's unified logging system.

By default support for the [log](https://docs.rs/log) crate is provided, but if
you would prefer just to use the lower level bindings you can disable the
default features.

When making use of targets (`info!(target: "t", "m");`), you should be aware
that a new log is allocated and stored in a map for the lifetime of the program.
I expect log allocations are extremely small, but haven't attempted to verify
it.

# Example

```rust
fn main() {
    OSLogger::new("com.example.test")
        .with_level(LevelFilter::Debug)
        .with_category("Settings", LevelFilter::Trace)
        .init()
        .unwrap();

    // Maps to OS_LOG_TYPE_DEBUG
    trace!(target: "Settings", "Trace");

    // Maps to OS_LOG_TYPE_INFO
    debug!("Debug");

    // Maps to OS_LOG_TYPE_DEFAULT
    info!(target: "Parsing", "Info");

    // Maps to OS_LOG_TYPE_ERROR
    warn!("Warn");

    // Maps to OS_LOG_TYPE_FAULT
    error!("Error");
}
```

# Missing features

* Activities
* Tracing
* Native support for line numbers and file names.
