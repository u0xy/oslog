# OSLog

<!-- cargo-sync-readme start -->

A minimal wrapper around Apple's [Unified Logging System]. On macOS, this
allows you to capture telemetry from your Rust programs for debugging and
performance analysis.


## Overview

This crate provides two facilities:

- the first one focuses on compatibility with the [log] crate and
thus provides only message logging to category loggers (named "targets" in
the [log] crate).
- the second one models as closely as it can the [Swift/ObjC OSLog API]:
this includes
  - usual message logging to category loggers,
  - activity tracing across categories
  - and [performance logging with signposts] for very detailed profiling
  with [Xcode Instruments].


### Choosing between the two facilities

To help you choose a facility, let's briefly explain logging features and
tools on macOS.

1. Both facilities can be used to make your program send log messages to
   the macOS Logging system. However, because the [`log`] crate only
   allows `&str` arguments, if you choose the `OSLogger` facility you will
   have to pay a small allocation price for each logged message (the
   conversion to `CString`). The `OSLog` facility only accepts `&CStr`
   arguments which is arguably better, and you can easily create safe
   a `CString` with the `cstr!` re-exported macro.

   In order to search and view the logged messages, note that, because
   macOS has deprecated sending logs to `/var/log` some time ago, you have
   to use the [log command line tool] or the [Console App] or one its
   [alternatives]. Besides the Apple official docs, this article, although
   somewhat dated, provides a [good overview of the Unified Logging
   System].

![Console App Window](https://raw.githubusercontent.com/u0xy/oslog/screenshots/screenshots/console-app.png)

2. The `OSLog` facility allows you to also trace [Activities] across
   categories. You can think of this as a semantic layer on top of
   potentially many concurrent messages logged.

![Console App Activities](https://raw.githubusercontent.com/u0xy/oslog/screenshots/screenshots/console-app-activities.png)

3. For performance analysis, the `OSLog` facility helps you instrument
   your code for profiling with "signpost" markers. Then you profile your
   program by running it in a specific run environment: on macOS, this
   environment is provided by the [`xctrace`] command line tool which
   generates a trace bundle. In practice, you run:

```sh
xcrun xctrace --template "Time Profiler" \
    --launch -- ./target/release/examples/simple-signposting
```

To make this even simpler, the Rust ecosystem provides the
[cargo-instruments] crate to do exactly this, with a nice Cargo
integration (and is available via `brew install cargo-instruments`):

```sh
cargo instruments -t "Time Profiler" --example simple-signposting
```

The trace bundle should be opened with [Xcode Instruments]

![Instruments Systrace Profiler](https://raw.githubusercontent.com/u0xy/oslog/screenshots/screenshots/instruments-system-trace.png)

With this in mind, you can choose which of the facilities you want to use.
My personal preference goes to the rich featured `OSLog` facility.


## Using `oslog::OSLogger` for `log` crate support

Because it depends on the `log` crate, this logger is available with the
`"logger"` feature.

As a reminder, on macOS, the logging levels are `Debug`, `Info`,
`Default`, `Error`, and `Fault`. They map to the level of the [log] crate
as follows

| OSLog level | log crate level |
| ---         | ---             |
| Debug       | Trace           |
| Info        | Debug           |
| Default     | Info            |
| Error       | Warn            |
| Fault       | Error           |

Here is a full example.

```rust
use oslog::OSLogger;
use log::{LevelFilter, trace, debug, info, warn, error};
OSLogger::new("com.example.test")
    .with_level(LevelFilter::Debug)
    .with_category("Settings", LevelFilter::Trace)
    .with_category("Parsing", LevelFilter::Info)
    .init()
    .unwrap();

trace!(target: "Settings", "Debug message to the `Settings` category");
debug!("Info message with no category");
info!(target: "Parsing", "Default message to the `Parsing` category");
warn!("Error message with no category");
error!("Fault message with no category");
```

Each category logger is allocated and stored in a map for the lifetime of
the program. If you care about not allocating during logging (as you
should), make sure you have created all category loggers beforehand, as
shown above.

Otherwise, if you log to a non-existing "target", the corresponding
category logger will be allocated once before logging. The allocations are
extremely small, but still. In the same vein, because you have to pass
`&str` messages instead of `&CStr`, a null-terminating `CString` will have
to be created for the FFI call to the C functions.


## Using `oslog::OSLog` for logging and profiling

Logging Quickstart

```rust
use oslog::{OSLog, cstr};

let log_settings = OSLog::new("com.example.test", "Settings");
//                             subsystem ~~^   category ~^

log_settings.debug(cstr!("This is a Debug message"));
log_settings.info(cstr!("This is an Info message"));
log_settings.default(cstr!("This is a Default message"));
log_settings.error(cstr!("This is an Error message"));
log_settings.fault(cstr!("This is a Fault message"));
```


### Performance analysis

For performance analysis, you need to profile your program using [Xcode
Instruments]. This can be done quite easily with the [cargo-instruments]
crate.


# Missing features

* Activities
* Tracing
* Native support for line numbers and file names.

[Unified Logging System]: https://developer.apple.com/documentation/os/logging
[Swift/ObjC OSLog API]: https://developer.apple.com/documentation/os/logging
[Console App]: https://support.apple.com/guide/console/welcome/mac
[alternatives]: https://eclecticlight.co/consolation-t2m2-and-log-utilities/
[Activities]: https://developer.apple.com/documentation/os/logging/collecting_log_messages_in_activities
[log command line tool]: https://developer.apple.com/documentation/os/logging/viewing_log_messages
[log]: https://docs.rs/log
[cargo-instruments]: https://crates.io/crates/cargo-instruments
[performance logging with signposts]: https://developer.apple.com/videos/play/wwdc2018/405/
[Xcode Instruments]: https://developer.apple.com/library/archive/documentation/ToolsLanguages/Conceptual/Xcode_Overview/MeasuringPerformance.html
[`xctrace`]: https://developer.apple.com/documentation/xcode-release-notes/xcode-12-release-notes
[good overview of the Unified Logging System]: https://eclecticlight.co/2018/03/19/macos-unified-log-1-why-what-and-how/


<!-- cargo-sync-readme end -->
