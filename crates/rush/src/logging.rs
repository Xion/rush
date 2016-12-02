//! Module implementing logging for the application.

use std::io::{self, Write};

use log::{Log, LogRecord, LogLevel, LogMetadata, set_logger, SetLoggerError};


const MAX_LEVEL: LogLevel = LogLevel::Trace;


pub fn init() -> Result<(), SetLoggerError> {
    set_logger(|max_log_level| {
        max_log_level.set(MAX_LEVEL.to_log_level_filter());
        Box::new(Logger)
    })
}


struct Logger;

impl Log for Logger {
    #[inline]
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= MAX_LEVEL
    }

    #[inline]
    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            writeln!(&mut io::stderr(),
                     "{}: {}", record.level(), record.args()).unwrap();
        }
    }
}
