use crate::OsLog;
use dashmap::DashMap;
use log::{LevelFilter, Log, Metadata, Record};

/// Defines a logger meant to be used with the
/// [log](https://crates.io/crates/log) crate.
/// Requires the "`logger`" feature.
///
/// As opposed to [`crate::OsLog`] and its [Swift/ObjC
/// counterpart](https://developer.apple.com/documentation/os/oslog), this
/// struct corresponds to one `subsystem` and several categories. This is
/// implemented by holding one logger per `category` along with its max level.
///
/// # Example
///
/// ```
/// use oslog::OsLogger;
/// use log::{LevelFilter};
/// OsLogger::new("com.example.oslog")
///     .with_level(LevelFilter::Trace)
///     .with_category("Settings", LevelFilter::Warn)
///     .with_category("Database", LevelFilter::Error)
///     .with_category("Database", LevelFilter::Trace)
///     .init()
///     .unwrap();
/// ```
pub struct OsLogger {
    subsystem: String,
    category_loggers: DashMap<String, (Option<LevelFilter>, OsLog)>,
}

/// Implement the [`log::Log`] trait for compatibility with the
/// [log](https://crates.io/crates/log) crate.
impl Log for OsLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        let max_level = self
            .category_loggers
            .get(metadata.target())
            .and_then(|pair| (*pair).0) // extract opt level
            .unwrap_or_else(log::max_level);

        metadata.level() <= max_level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let pair = self
                .category_loggers
                .entry(record.target().into())
                .or_insert((None, OsLog::new(&self.subsystem, record.target())));

            let message = std::format!("{}", record.args());
            (*pair).1.with_level(record.level().into(), &message);
        }
    }

    fn flush(&self) {}
}

impl From<log::Level> for crate::Level {
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

/// Builder API for constructing an `OsLogger`.
///
impl OsLogger {
    /// Creates a new logger using the Builder Pattern.
    ///
    /// Notes:
    ///
    /// - By default the level filter will be set to `LevelFilter::Trace`, see
    /// [`with_level()`].
    /// - You *should* add category loggers using [`with_category()`].
    /// - You *must* call [`init()`] to finalize the set up.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use oslog::OsLogger;
    /// use log::{LevelFilter};
    /// OsLogger::new("com.example.oslog")
    ///     .with_level(LevelFilter::Trace)
    ///     .with_category("Settings", LevelFilter::Warn)
    ///     .with_category("Database", LevelFilter::Error)
    ///     .with_category("Database", LevelFilter::Trace)
    ///     .init()
    ///     .unwrap();
    /// ```
    ///
    /// [`with_level()`]: #method.with_level
    /// [`with_category()`]: #method.with_category
    /// [`init()`]: #method.init
    #[must_use = "You must call init() to begin logging"]
    pub fn new(subsystem: &str) -> Self {
        Self {
            subsystem: subsystem.to_string(),
            category_loggers: DashMap::new(),
        }
    }

    /// Specifies that only levels at or above `max_level` will be logged.
    ///
    /// # Example
    ///
    /// ```
    /// use oslog::OsLogger;
    /// use log::{LevelFilter};
    /// OsLogger::new("com.example.oslog")
    ///     .with_level(LevelFilter::Info)
    ///     .with_category("Settings", LevelFilter::Trace)
    ///     .init()
    ///     .unwrap();
    /// ```
    pub fn with_level(self, max_level: LevelFilter) -> Self {
        log::set_max_level(max_level);
        self
    }

    /// Sets or updates the category's level filter. A new logger is
    /// introduced the first time the category is declared, otherwise the
    /// existing logger is reused.
    ///
    /// See [`new()`] for an example.
    ///
    /// [`new()`]: #method.new
    pub fn with_category(self, category: &str, level: LevelFilter) -> Self {
        self.category_loggers
            .entry(category.into())
            .and_modify(|(existing_level, _)| *existing_level = Some(level))
            .or_insert((Some(level), OsLog::new(&self.subsystem, category)));

        self
    }

    /// Instantiate the actual logger and configure the
    /// [log](https://crates.io/crates/log) crate to use it when using calls
    /// such as `info!(...)`.
    ///
    /// # Note
    ///
    /// This method _must_ be called in order for the logger to be effective.
    pub fn init(self) -> Result<(), log::SetLoggerError> {
        log::set_boxed_logger(Box::new(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::{debug, error, info, trace, warn};

    #[test]
    fn test_basic_usage() {
        OsLogger::new("com.example.oslog")
            .with_level(LevelFilter::Trace)
            .with_category("Settings", LevelFilter::Warn)
            .with_category("Database", LevelFilter::Error)
            .with_category("Database", LevelFilter::Trace)
            .init()
            .unwrap();

        // This will not be logged because of its category's custom level filter.
        info!(target: "Settings", "Info");

        warn!(target: "Settings", "Warn");
        error!(target: "Settings", "Error");

        trace!("Trace");
        debug!("Debug");
        info!("Info");
        warn!(target: "Database", "Warn");
        error!("Error");
    }
}
