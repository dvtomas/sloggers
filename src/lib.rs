//! This crate provides frequently used
//! [slog](https://github.com/slog-rs/slog) loggers and convenient functions.
//!
//! # Examples
//!
//! Creates a logger via `TerminalLoggerBuilder`:
//!
//! ```
//! #[macro_use]
//! extern crate slog;
//! extern crate sloggers;
//!
//! use sloggers::Build;
//! use sloggers::terminal::{TerminalLoggerBuilder, Destination};
//! use sloggers::types::{FilterConfig, Severity};
//!
//! # fn main() {
//! let mut builder = TerminalLoggerBuilder::new();
//! builder.filter_config(FilterConfig::always_pass_on_severity_at_least(Severity::Debug));
//! builder.destination(Destination::Stderr);
//!
//! let logger = builder.build().unwrap();
//! info!(logger, "Hello World!");
//! # }
//! ```
//!
//! Creates a logger from configuration text (TOML):
//!
//! ```
//! #[macro_use]
//! extern crate slog;
//! extern crate sloggers;
//! extern crate serdeconv;
//!
//! use sloggers::{Config, LoggerConfig};
//!
//! # fn main() {
//! let config: LoggerConfig = serdeconv::from_toml_str(r#"
//! type = "terminal"
//! format = "full"
//! source_location = "module_and_line"
//! timezone = "local"
//! destination = "stdout"
//! channel_size = 0
//! evaluation_order = "LoggerAndMessage"
//!
//! [filter_config]
//! type = "PassOnAnyOf"
//! always_pass_on_severity_at_least = "debug"
//!
//! [[filter_config.passes]]
//! keys_and_values = [["system", "SystemA"], ["subsystem", "SubsystemAB"]]
//! severity_at_least = "trace"
//!
//! [[filter_config.passes]]
//! keys_and_values = [["system", "SystemC"]]
//! severity_at_least = "debug"
//! "#).unwrap();
//!
//! let logger = config.build_logger().unwrap();
//! info!(logger, "Hello World!");
//! # }
//! ```
#![warn(missing_docs)]
extern crate chrono;
extern crate libflate;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_kvfilter;
extern crate slog_scope;
extern crate slog_stdlog;
extern crate slog_term;
#[cfg(test)]
extern crate tempdir;
#[macro_use]
extern crate trackable;

pub use build::{Build, LoggerBuilder};
pub use config::{Config, LoggerConfig};
pub use error::{Error, ErrorKind};
pub use misc::set_stdlog_logger;

pub mod file;
pub mod null;
pub mod terminal;
pub mod types;

mod build;
mod config;
mod error;
mod misc;

/// A specialized `Result` type for this crate.
pub type Result<T> = ::std::result::Result<T, Error>;
