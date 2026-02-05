use core::ptr::null_mut;

use crate::{BLOCKED_QUEUE, BUFFER, log, logs::LogLevel};

use super::{
    TASK_HANDLER, Task, TaskBlockControl, TaskState,
    list::{task_list_get_task_by_pid, task_list_update_task_by_pid},
    task_pid,
};

/// Block the current task until the given tick is reach.
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

/// Pop the oldest element in the blocked queue and check if the task can be awake. If not, repush
/// it to the blocked queue
/// TODO: Use a better data structure than a RingBuffer for the blocked queue.
pub fn task_awake_blocked(tick: usize) {
    #[allow(static_mut_refs)]
    let pid = unsafe { BLOCKED_QUEUE.pop() };
    if pid.is_none() {
        log!(LogLevel::Error, "Error getting the oldest pid in run queue");
        return;
    }
    let task = task_list_get_task_by_pid(pid.expect("Error getting the pid behind the Option<>"));
    if task.is_none() {
        log!(
            LogLevel::Error,
            "Error getting the task by pid, the task may not exist"
        );
        return;
    }
    // Allow expected, we check the value before, if it's some, there's shouldn't be any problem by
    // unwrapping it.
    #[allow(clippy::expect_used)]
    match task
        .expect("Failed to get the task behind the Option<>. This shouldn't be possible")
        .block_control
    {
        TaskBlockControl::AwakeTick(awake_tick) => {
            log!(
                LogLevel::Debug,
                "\n\nHERE IT SHOULD AWAKE: awake_tick: {}\ttick: {}\n\n",
                awake_tick,
                tick
            );
            if tick >= awake_tick {
                // push to run queue
                log!(LogLevel::Debug, "\n\nHERE IT SHOULD AWAKE\n\n");
                #[allow(static_mut_refs)]
                unsafe {
                    BUFFER.push(pid.expect("Failed to get the pid behind the Option<>"));
                };
                return;
            } else {
                // push to blocked queue
                #[allow(static_mut_refs)]
                unsafe {
                    BLOCKED_QUEUE.push(pid.expect("Failed to get the pid behind the Option<>"))
                };
                return;
            }
        }
        TaskBlockControl::None => return,
    }
}
