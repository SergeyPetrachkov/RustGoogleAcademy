pub trait Logger {
    /// Log a message at the given verbosity level.
    fn log(&self, verbosity: u8, message: &str);
}

struct StderrLogger;

impl Logger for StderrLogger {
    fn log(&self, verbosity: u8, message: &str) {
        eprintln!("verbosity={verbosity}: {message}");
    }
}

struct Filter<L: Logger> {
    logger: L,
    should_log: fn(u8, &str) -> bool,
}

impl<L: Logger> Filter<L> {
    fn new(logger: L, should_log: fn(u8, &str) -> bool) -> Filter<L> {
        Filter { logger, should_log }
    }

    fn log(&self, verbosity: u8, message: &str) {
        if (self.should_log)(verbosity, message) {
            self.logger.log(verbosity, message);
        }
    }
}

fn main() {
    let logger = Filter::new(StderrLogger, |_verbosity, msg| msg.contains("yikes"));
    logger.log(5, "FYI");
    logger.log(1, "yikes, something went wrong");
    logger.log(2, "uhoh");
}