use slog::Logger;
use slog_async::AsyncGuard;

use file::FileLoggerBuilder;
use null::NullLoggerBuilder;
use Result;
use terminal::TerminalLoggerBuilder;

/// This trait allows to build a logger instance.
pub trait Build {
    /// Builds a logger.
    fn build(&self) -> Result<(Logger, Option<AsyncGuard>)>;
}

#[derive(Debug)]
pub enum LoggerBuilder {
    File(FileLoggerBuilder),
    Null(NullLoggerBuilder),
    Terminal(TerminalLoggerBuilder),
}

impl Build for LoggerBuilder {
    fn build(&self) -> Result<(Logger, Option<AsyncGuard>)> {
        match *self {
            LoggerBuilder::File(ref b) => track!(b.build()),
            LoggerBuilder::Null(ref b) => track!(b.build()),
            LoggerBuilder::Terminal(ref b) => track!(b.build()),
        }
    }
}
