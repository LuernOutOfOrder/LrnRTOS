use core::ptr::null_mut;

use crate::log;
use crate::{
    ktime::tick::GLOBAL_TICK,
    logs::{self, LogLevel},
    task::{TASK_HANDLER, Task},
};

unsafe extern "C" {
    pub fn sleep(tick: usize);
}

// Use no mangle because this function is called from an asm function
#[unsafe(no_mangle)]
fn task_set_wake_tick(tick: usize) {
    let current_task: *mut Task = unsafe { TASK_HANDLER };
    if current_task == null_mut() {
        log!(
            LogLevel::Error,
            "Error getting the current task, invariant violated. Sleep couldn't be used outside of a task."
        );
        // See how to handle this, what to return or something else.
    }
    let current_tick = unsafe { GLOBAL_TICK };
    let awake_tick = current_tick + tick;
    // Update task
    // Call reschedule or return and continue in sleep fn
}
