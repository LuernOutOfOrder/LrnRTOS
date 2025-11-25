// Actually used when logs feature is enabled
use crate::config::LOG_LEVEL;
#[allow(unused)]
use crate::print;

#[derive(PartialEq, PartialOrd)]
/// Enum for logging level
pub enum LogLevel {
    // Debug, lot of logs, possibly exploded kernel size
    Debug,
    // Basic info, system living flow
    Info,
    // Warning, like something not right but it's ok
    Warn,
    // Error, possibly recoverable but not something to mess with i guess
    Error,
}

/// Main log function, used for all logs, call to print! macro inside to avoid repeating codes. Use
/// hexadecimal escape code to make prefix log in color
///
/// Params:
/// level: use LogLevel enum to define which logging level used.
/// msg: the message to print as an &str
pub fn log(level: LogLevel, msg: &str) {
    if level >= LOG_LEVEL {
        match level {
            LogLevel::Info => print!("\x1b[32;1m[INFO]\x1b[0m {}\n", msg),
            LogLevel::Debug => print!("\x1b[35;1m[DEBUG]\x1b[0m {}\n", msg),
            LogLevel::Warn => print!("\x1b[33;1m[WARNING]\x1b[0m {}\n", msg),
            LogLevel::Error => {
                print!("\x1b[31;1m[ERROR]\x1b[0m {}\n", msg);
            }
        }
    }
}

// Log macro enabled by features. Avoid using feature on all module.

#[cfg(feature = "logs")]
#[macro_export]
macro_rules! log {
    ($log_level:expr, $msg:expr) => {
        $crate::logs::log($log_level, $msg);
    };
}

#[cfg(not(feature = "logs"))]
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {};
}
