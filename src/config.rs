// Config file where all static is defined. 
// Use to modify the behaviour of the kernel. Like the scheduler time or logs level.

use crate::logs::LogLevel;

// Define the duration between each tick in ms.
// 1 = 1ms
pub static TICK_DURATION: u64 = 4;

// Static for log level, everything equal to this or below will be logged
pub static LOG_LEVEL: LogLevel = LogLevel::Debug;
