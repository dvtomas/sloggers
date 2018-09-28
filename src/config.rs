use slog::Logger;

use file::FileLoggerConfig;
use null::NullLoggerConfig;
use terminal::TerminalLoggerConfig;
use {Build, LoggerBuilder, Result};

/// Configuration of a logger builder.
pub trait Config {
    /// Logger builder.
    type Builder: Build;

    /// Makes a logger builder associated with this configuration.
    fn try_to_builder(&self) -> Result<Self::Builder>;

    /// Builds a logger with this configuration.
    fn build_logger(&self) -> Result<Logger> {
        let builder = track!(self.try_to_builder())?;
        let logger = track!(builder.build())?;
        Ok(logger)
    }
}

/// The configuration of `LoggerBuilder`.
///
/// # Examples
///
/// Null logger.
///
/// ```
/// extern crate sloggers;
/// extern crate serdeconv;
///
/// use sloggers::LoggerConfig;
///
/// # fn main() {
/// let toml = r#"
/// type = "null"
/// "#;
/// let _config: LoggerConfig = serdeconv::from_toml_str(toml).unwrap();
/// # }
/// ```
///
/// Terminal logger.
///
/// ```
/// extern crate sloggers;
/// extern crate serdeconv;
///
/// use sloggers::LoggerConfig;
///
/// # fn main() {
/// let toml = r#"
/// type = "terminal"
/// format = "full"
/// source_location = "module_and_line"
/// timezone = "local"
/// destination = "stdout"
/// channel_size = 0
/// evaluation_order = "LoggerAndMessage"
///
/// [filter_config]
/// type = "PassOnAnyOf"
/// always_pass_on_severity_at_least = "info"
///
/// [[filter_config.passes]]
/// key = "key1"
/// value = "value1"
/// severity_at_least = "trace"
///
/// [[filter_config.passes]]
/// key = "key2"
/// value = "value2"
/// severity_at_least = "debug"
/// "#;
/// let _config: LoggerConfig = serdeconv::from_toml_str(toml).unwrap();
/// # }
/// ```
///
/// File logger.
///
/// ```
/// extern crate sloggers;
/// extern crate serdeconv;
///
/// use sloggers::LoggerConfig;
///
/// # fn main() {
/// let toml = r#"
/// type = "file"
/// format = "full"
/// source_location = "module_and_line"
/// timezone = "local"
/// timestamp_template = "%Y%m%d_%H%M"
/// path = ""
/// channel_size = 1024
/// truncate = false
/// rotate_size = 9223372036854775807
/// rotate_keep = 8
/// rotate_compress = false
/// evaluation_order = "LoggerAndMessage"
///
/// [filter_config]
/// type = "PassOnAnyOf"
/// always_pass_on_severity_at_least = "info"
///
/// [[filter_config.passes]]
/// key = "key1"
/// value = "value1"
/// severity_at_least = "trace"
///
/// [[filter_config.passes]]
/// key = "key2"
/// value = "value2"
/// severity_at_least = "debug"
/// "#;
/// let _config: LoggerConfig = serdeconv::from_toml_str(toml).unwrap();
/// # }
/// ```
#[allow(missing_docs)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum LoggerConfig {
    File(FileLoggerConfig),
    Null(NullLoggerConfig),
    Terminal(TerminalLoggerConfig),
}

impl Config for LoggerConfig {
    type Builder = LoggerBuilder;
    fn try_to_builder(&self) -> Result<Self::Builder> {
        match *self {
            LoggerConfig::File(ref c) => track!(c.try_to_builder()).map(LoggerBuilder::File),
            LoggerConfig::Null(ref c) => track!(c.try_to_builder()).map(LoggerBuilder::Null),
            LoggerConfig::Terminal(ref c) => {
                track!(c.try_to_builder()).map(LoggerBuilder::Terminal)
            }
        }
    }
}

impl Default for LoggerConfig {
    fn default() -> Self {
        LoggerConfig::Terminal(TerminalLoggerConfig::default())
    }
}

#[cfg(test)]
mod tests {
    extern crate serdeconv;

    use terminal::TerminalLoggerConfig;
    use types::{FilterConfig, PassIfMatch, Severity};
    use LoggerConfig;
    use file::FileLoggerConfig;

    fn sample_filter_config() -> FilterConfig {
        FilterConfig::PassOnAnyOf {
            always_pass_on_severity_at_least: Severity::Info,
            passes: vec![
                PassIfMatch::new("key1", "value1", Severity::Trace),
                PassIfMatch::new("key2", "value2", Severity::Debug),
            ],
        }
    }

    #[test]
    fn test_terminal_config() {
        let mut terminal_logger_config = TerminalLoggerConfig::default();
        terminal_logger_config.filter_config = sample_filter_config();
        let config: LoggerConfig = LoggerConfig::Terminal(terminal_logger_config);

        let config_string = serdeconv::to_toml_string(&config).unwrap();
//        eprintln!("{}", config_string);

        let config_again: LoggerConfig = serdeconv::from_toml_str(&config_string).unwrap();
        assert_eq!(config_again, config);
    }

    #[test]
    fn test_file_config() {
        let mut file_logger_config = FileLoggerConfig::default();
        file_logger_config.filter_config = sample_filter_config();
        let config: LoggerConfig = LoggerConfig::File(file_logger_config);

        let config_string = serdeconv::to_toml_string(&config).unwrap();
//        eprintln!("{}", config_string);

        let config_again: LoggerConfig = serdeconv::from_toml_str(&config_string).unwrap();
        assert_eq!(config_again, config);
    }
}
