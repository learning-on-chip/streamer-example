use log::{self, Log, LogLevel, LogMetadata, LogRecord};
use term;

pub struct Logger(LogLevel);

impl Logger {
    #[inline]
    pub fn new(level: LogLevel) -> Logger {
        Logger(level)
    }

    #[allow(unused_must_use)]
    pub fn activate(self) {
        log::set_logger(|max_log_level| {
            max_log_level.set(self.0.to_log_level_filter());
            Box::new(self)
        });
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= self.0
    }

    #[allow(unused_must_use)]
    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            if let Some(mut output) = term::stdout() {
                if record.metadata().level() < LogLevel::Info {
                    output.fg(term::color::RED);
                } else {
                    output.fg(term::color::GREEN);
                }
                write!(output, "{:>12}", record.target());
                output.reset();
                write!(output, " {}\n", record.args());
            }
        }
    }
}
