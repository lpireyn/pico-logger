// SPDX-License-Identifier: MIT OR Apache-2.0

//! Demo of Pico Logger.

fn main() {
    // Initialize Pico Logger
    pico_logger::init(log::LevelFilter::max());

    // Log a message at each log level
    log::error!("this is an error message");
    log::warn!("this is a warning message");
    log::info!("this is an info message");
    log::debug!("this is a debug message");
    log::trace!("this is a trace message");
}
