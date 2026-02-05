use core::ptr::null_mut;

use crate::{BLOCKED_QUEUE, log, logs::LogLevel};

use super::{
    TASK_HANDLER, Task, TaskBlockControl, TaskState, list::task_list_update_task_by_pid, task_pid,
};

pub fn task_block_until(tick: usize) {
    let current_task: *mut Task = unsafe { TASK_HANDLER };
    if current_task == null_mut() {
        log!(
            LogLevel::Error,
            "Error getting the current task, invariant violated. Sleep couldn't be used outside of a task."
        );
        // See how to handle this, what to return or something else.
    }
    let mut current_task_deref: Task = unsafe { *current_task };
    // Update task
    // Update current state to block
    current_task_deref.state = TaskState::Blocked;
    // Update block control and pass the awake_tick to it
    current_task_deref.block_control = TaskBlockControl::AwakeTick(tick);
    // Update task and push pid to block queue
    let pid = task_pid(&current_task_deref);
    task_list_update_task_by_pid(pid, current_task_deref);
    #[allow(static_mut_refs)]
    unsafe {
        BLOCKED_QUEUE.push(pid);
    }
}
