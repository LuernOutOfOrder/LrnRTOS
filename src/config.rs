// Config file where all static is defined.
// Use to modify the behaviour of the kernel. Like the scheduler time or logs level.

use crate::logs::LogLevel;

// Define the duration between each tick in ms.
// 1 = 1ms
pub static TICK_DURATION: u64 = 4;

// Define the safety tick in kernel boot, used to avoid trigger an interrupt when the kernel is
// booting
// 1 = 1 seconds
pub static TICK_SAFETY_DURATION: u64 = 1;

// Static for log level, everything equal to this or below will be logged
pub static LOG_LEVEL: LogLevel = LogLevel::Debug;

// Define the uart address to use in kprint
pub static KPRINT_ADDRESS: usize = 0x1000_0000;

// ————————————————————————————————————————————————————————————
// ———————— Define the max size of devices sub-systems ————————
// ————————————————————————————————————————————————————————————
pub static CPU_INTC_MAX_SIZE: usize = 2;
pub static TIMER_MAX_SIZE: usize = 2;
pub static SERIAL_MAX_SIZE: usize = 4;

// ————————————————————————————————————————————————————————————
// ————————————— Define the max size of fdt pool ——————————————
// ————————————————————————————————————————————————————————————
pub static FDT_MAX_STACK: usize = 64;
pub static FDT_MAX_PROPS: usize = 128;

// ————————————————————————————————————————————————————————————
// ————————————— Define the max size of Task list —————————————
// ————————————————————————————————————————————————————————————
pub static TASK_LIST_MAX_SIZE: usize = 4;
// ————————————————————————————————————————————————————————————
// ————————————— Define the max size of the Run queue —————————
// ————————————————————————————————————————————————————————————
// The run queue is len - 1, if the size is 4, it will only use 3 slot in the queue.
pub static RUN_QUEUE_MAX_SIZE: usize = 3;
// ————————————————————————————————————————————————————————————
// ————————————— Define the number of CPU core ————————————————
// ————————————————————————————————————————————————————————————
pub static CPU_CORE_NUMBER: usize = 1;

// Kernel stack size
// WARNING
// Changing the kernel stack size can cause a lot of error, UB, or just break everything's
// don't touch this unless you know what you do
pub static KERNEL_STACK_SIZE: usize = 0x4000;
