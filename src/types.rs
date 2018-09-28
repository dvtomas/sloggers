//! Commonly used types.
use std::str::FromStr;

use slog::{Drain, Level, LevelFilter};
use slog_kvfilter::FilterSpec;
use {Error, ErrorKind};

/// The severity of a log record.
///
/// # Examples
///
/// The default value:
///
/// ```
/// use sloggers::types::Severity;
///
/// assert_eq!(Severity::default(), Severity::Info);
/// ```
///
/// # Notice
///
/// By default, `slog` disables trace level logging in debug builds,
/// and trace and debug level logging in release builds.
/// For enabling them, you need to specify some features (e.g, `max_level_trace`) to `slog`.
///
/// See [slog's documentation](https://docs.rs/slog/2.2.3/slog/#notable-details) for more details.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Trace,
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

impl Severity {
    /// Converts `Severity` to `Level`.
    pub fn as_level(&self) -> Level {
        match *self {
            Severity::Trace => Level::Trace,
            Severity::Debug => Level::Debug,
            Severity::Info => Level::Info,
            Severity::Warning => Level::Warning,
            Severity::Error => Level::Error,
            Severity::Critical => Level::Critical,
        }
    }

    /// Sets `LevelFilter` to `drain`.
    pub fn set_level_filter<D: Drain>(&self, drain: D) -> LevelFilter<D> {
        LevelFilter::new(drain, self.as_level())
    }
}

impl Default for Severity {
    fn default() -> Self {
        Severity::Info
    }
}

impl FromStr for Severity {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "trace" => Ok(Severity::Trace),
            "debug" => Ok(Severity::Debug),
            "info" => Ok(Severity::Info),
            "warning" => Ok(Severity::Warning),
            "error" => Ok(Severity::Error),
            "critical" => Ok(Severity::Critical),
            _ => track_panic!(ErrorKind::Invalid, "Undefined severity: {:?}", s),
        }
    }
}

/// The format of log records.
///
/// # Examples
///
/// The default value:
///
/// ```
/// use sloggers::types::Format;
///
/// assert_eq!(Format::default(), Format::Full);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Format {
    /// Full format.
    Full,

    /// Compact format.
    Compact,
}

impl Default for Format {
    fn default() -> Self {
        Format::Full
    }
}

impl FromStr for Format {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "full" => Ok(Format::Full),
            "compact" => Ok(Format::Compact),
            _ => track_panic!(ErrorKind::Invalid, "Undefined log format: {:?}", s),
        }
    }
}

/// Time Zone.
///
/// # Examples
///
/// The default value:
///
/// ```
/// use sloggers::types::TimeZone;
///
/// assert_eq!(TimeZone::default(), TimeZone::Local);
/// ```
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TimeZone {
    Utc,
    Local,
}

impl Default for TimeZone {
    fn default() -> Self {
        TimeZone::Local
    }
}

impl FromStr for TimeZone {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "utc" => Ok(TimeZone::Utc),
            "local" => Ok(TimeZone::Local),
            _ => track_panic!(ErrorKind::Invalid, "Undefined time zone: {:?}", s),
        }
    }
}

/// Source Location.
///
/// # Examples
///
/// The default value:
///
/// ```
/// use sloggers::types::SourceLocation;
///
/// assert_eq!(SourceLocation::default(), SourceLocation::ModuleAndLine);
/// ```
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceLocation {
    None,
    ModuleAndLine,
}

impl Default for SourceLocation {
    fn default() -> Self {
        SourceLocation::ModuleAndLine
    }
}

impl FromStr for SourceLocation {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "none" => Ok(SourceLocation::None),
            "module_and_line" => Ok(SourceLocation::ModuleAndLine),
            _ => track_panic!(
                ErrorKind::Invalid,
                "Undefined source code location: {:?}",
                s
            ),
        }
    }
}

/// Pass a message if: For any entry in `keys_and_values` all the keys and values
/// in that entry have corresponding keys and values in the message AND the
/// severity for that entry is at least `severity_at_least`
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct PassIfMatch {
    /// Key-value pairs that must all match in the key-value pair
    pub keys_and_values: Vec<(String, String)>,
    /// Severity must be at least this in order for the message to pass
    pub severity_at_least: Severity,
}

impl PassIfMatch {
    /// Creates a new PassIfMatch struct
    pub fn new(keys_and_values: &[(impl ToString, impl ToString)], severity: Severity) -> Self {
        PassIfMatch {
            keys_and_values: keys_and_values
                .iter()
                .map(|(key, value)| (key.to_string(), value.to_string()))
                .collect(),
            severity_at_least: severity,
        }
    }

    /// Builds a `FilterSpec` from this struct
    pub fn to_filter_spec(&self) -> FilterSpec {
        let match_filters: Vec<_> = self.keys_and_values
            .iter()
            .map(|(key, value)| FilterSpec::match_kv(key, value))
            .collect();

        FilterSpec::LevelAtLeast(self.severity_at_least.as_level())
            .and(FilterSpec::all_of(&match_filters))
    }
}

/// A structure for simplified building of common KVFilter spec scenarios.
#[serde(tag = "type")]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum FilterConfig {
    /// Pass all messages with severity at least `always_pass_on_severity_at_least`
    /// Also pass any message that matches all the Key and Value pairs of any
    /// of the variants if the message has at least a severity for a given variant
    PassOnAnyOf {
        /// A message will pass if it's level is at least this
        always_pass_on_severity_at_least: Severity,
        /// A message will pass if it matches any of the variants
        passes: Vec<PassIfMatch>,
    },
    /// You can build any FilterSpec you want with this setting. See `FilterSpec` docs for details.
    /// Basically you can build arbitrary Bool logic expressions.
    Custom {
        /// Arbitrary configuration of a filter. There may be problems serializing these specifications to TOML,
        /// consider using JSON when using `FilterConfig::Custom`.
        filter_spec: FilterSpec,
    },
}

impl FilterConfig {
    /// Constructs a FilterConfig that will make a message pass if it's severity is at least the specified one,
    /// without any exceptions.
    pub fn always_pass_on_severity_at_least(severity: Severity) -> Self {
        FilterConfig::PassOnAnyOf {
            always_pass_on_severity_at_least: severity,
            passes: Vec::new(),
        }
    }
}

impl Default for FilterConfig {
    fn default() -> Self {
        FilterConfig::PassOnAnyOf {
            always_pass_on_severity_at_least: Severity::default(),
            passes: Vec::new(),
        }
    }
}

impl FilterConfig {
    /// Converts this config into a `FilterSpec`
    pub fn to_filter_spec(&self) -> FilterSpec {
        match self {
            FilterConfig::PassOnAnyOf {
                passes,
                always_pass_on_severity_at_least,
            } => {
                let variant_filters: Vec<_> =
                    passes.iter().map(|elem| elem.to_filter_spec()).collect();
                FilterSpec::LevelAtLeast(always_pass_on_severity_at_least.as_level())
                    .or(FilterSpec::any_of(&variant_filters))
            }
            FilterConfig::Custom { filter_spec } => filter_spec.clone(),
        }
    }
}
