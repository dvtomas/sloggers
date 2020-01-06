use std::fmt::Debug;
use std::io;

use slog::{Drain, FnValue, Logger, Record};
use slog_async::{Async, AsyncGuard};
use slog_kvfilter::{KVFilter, KVFilterConfig};
use slog_term;

use types::{SourceLocation, TimeZone};

pub fn module_and_line(record: &Record) -> String {
    format!("{}:{}", record.module(), record.line())
}

pub fn timezone_to_timestamp_fn(timezone: TimeZone) -> fn(&mut dyn io::Write) -> io::Result<()> {
    match timezone {
        TimeZone::Utc => slog_term::timestamp_utc,
        TimeZone::Local => slog_term::timestamp_local,
    }
}


pub fn build_with_drain<D>(
    drain: D,
    channel_size: usize,
    kv_filter_config: KVFilterConfig,
    source_location: SourceLocation,
) -> (Logger, Option<AsyncGuard>)
    where
        D: Drain + Send + 'static,
        D::Err: Debug,
{

    // async inside, level and key value filters outside for speed
    let (drain, guard) = {
        let (drain, guard) = Async::new(drain.fuse()).chan_size(channel_size).build_with_guard();
        (drain.fuse(), guard)
    };

    let kv_filter = KVFilter::new_from_config(drain, kv_filter_config);

    let logger = match source_location {
        SourceLocation::None => Logger::root(kv_filter.fuse(), o!()),
        SourceLocation::ModuleAndLine => {
            Logger::root(kv_filter.fuse(), o!("module" => FnValue(module_and_line)))
        }
    };

    (logger, Some(guard))
}

