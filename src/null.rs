//! Null logger.
use slog::{Discard, Logger};
use slog_async::AsyncGuard;

use {Build, Config, Result};

/// Null logger builder.
///
/// This will create a logger which discards all log records.
#[derive(Debug)]
pub struct NullLoggerBuilder;

impl Build for NullLoggerBuilder {
    fn build(&self) -> Result<(Logger, Option<AsyncGuard>)> {
        let logger = Logger::root(Discard, o!());
        Ok((logger, None))
    }
}

/// The configuration of `NullLoggerBuilder`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NullLoggerConfig {}

impl Config for NullLoggerConfig {
    type Builder = NullLoggerBuilder;
    fn try_to_builder(&self) -> Result<Self::Builder> {
        Ok(NullLoggerBuilder)
    }
}
