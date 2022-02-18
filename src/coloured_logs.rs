use log::Level;

pub const RESET: &str = "\x1B[0m";
pub const SUBTLE: &str = "\x1B[30;1m";

pub fn get_colour(level: log::Level) -> &'static str {
    match level {
        Level::Trace => "\x1B[36m",   // Cyan
        Level::Debug => "\x1B[34m",   // Blue
        Level::Info => "\x1B[32m",    // Green
        Level::Warn => "\x1B[33m",    // Yellow
        Level::Error => "\x1B[1;31m", // Bold red
    }
}
