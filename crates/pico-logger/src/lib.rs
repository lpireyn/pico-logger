// SPDX-License-Identifier: MIT OR Apache-2.0

//! A minuscule logger that simply prints the log messages to stderr.
//! The only configurable element is the maximum log level.
//!
//! # Usage
//!
//! Early during execution, call the [`init`] function with the maximum log level:
//!
//! ``` rust
//! pico_logger::init(log::LevelFilter::Info);
//! ```
//!

use log::{Level, LevelFilter, Log, Metadata, Record};

/// Initializes Pico Logger with the given maximum log level.
///
/// # Example
///
/// ``` rust
/// pico_logger::init(log::LevelFilter::Info);
/// ```
///
/// # Panics
///
/// This function panics if a logger has already been installed
/// (see [the `log` crate documentation](https://docs.rs/log/latest/log/index.html#warning)).
pub fn init(max_level: LevelFilter) {
    let logger = PicoLogger { max_level };
    // NOTE: `SetLoggerError` does *not* implement `Error`
    log::set_boxed_logger(Box::new(logger)).expect("logger already installed");
    log::set_max_level(max_level);
}

/// Pico logger.
#[derive(Debug)]
struct PicoLogger {
    max_level: LevelFilter,
}

impl Log for PicoLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.max_level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            eprintln!("{}: {}", level_label(record.level()), record.args());
        }
    }

    fn flush(&self) {}
}

fn level_label(level: Level) -> &'static str {
    match level {
        Level::Error => "error",
        Level::Warn => "warning",
        Level::Info => "info",
        Level::Debug => "debug",
        Level::Trace => "trace",
    }
}
