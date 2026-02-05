use core::ptr::null_mut;

use crate::scheduler::dispatch;
use crate::{BLOCKED_QUEUE, log};
use crate::{
    ktime::tick::GLOBAL_TICK,
    logs::LogLevel,
    task::{TASK_HANDLER, Task},
};

use super::list::task_list_update_task_by_pid;
use super::primitives::task_block_until;
use super::{TaskBlockControl, TaskState, task_pid};

unsafe extern "C" {
    // Put the current task to sleep until the number of tick given is passed
    // tick: the number of tick the task need to sleep.
    pub fn sleep(tick: usize);
}

// Use no mangle because this function is called from an asm function
#[unsafe(no_mangle)]
fn task_set_wake_tick(tick: usize) {
    let current_tick = unsafe { GLOBAL_TICK };
    let awake_tick = current_tick + tick;
    // Call task primitive to update current task state
    task_block_until(awake_tick);
    // Call a re-schedule
    dispatch();
}
