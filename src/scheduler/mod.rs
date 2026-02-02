/*
File info: Scheduler main file

Test coverage: ...

Tested:

Not tested:

Reasons: Not even really implemented so there's no need to test something that doesn't even consider finish

Tests files:

References:
*/

use crate::{
    BUFFER,
    arch::scheduler::{SCHEDULER_CTX, SchedulerCtx, sched_ctx_restore},
    log,
    logs::LogLevel,
    task::{
        TASK_HANDLER, TaskState,
        list::{task_list_get_task_by_pid, task_list_update_task_by_pid},
        task_context_switch, task_pid,
    },
};

/// Temporary function use to test the context switch and context restore on multiple task.
/// Will certainly be used later on the real scheduler.
/// Pop oldest task from RingBuffer, save the task context, update it, and repush it to the
/// RingBuffer.
/// Read on the RingBuffer to get the next task, update it, and update the RingBuffer.
/// Not the best way to use the RingBuffer but it will do.
#[unsafe(no_mangle)]
pub fn dispatch() {
    // Current running task
    let mut current_task = unsafe { *TASK_HANDLER };
    current_task.state = TaskState::Ready;
    let pid = task_pid(&current_task);
    task_list_update_task_by_pid(pid, current_task);
    #[allow(static_mut_refs)]
    unsafe {
        BUFFER.push(pid)
    };
    // Update and load next task
    #[allow(static_mut_refs)]
    let get_next_task = unsafe { BUFFER.pop() };
    if get_next_task.is_none() {
        log!(
            LogLevel::Error,
            "Error getting the last task from RingBuffer"
        );
    }
    let next_task_pid = get_next_task.unwrap();
    let next_task = task_list_get_task_by_pid(next_task_pid).unwrap();
    next_task.state = TaskState::Running;
    task_list_update_task_by_pid(next_task_pid, *next_task);
    // WARN: This can break?
    unsafe { TASK_HANDLER = next_task }
    task_context_switch(next_task);
}

pub fn switch_scheduler_ctx() {
    #[allow(static_mut_refs)]
    let ctx = unsafe { &mut SCHEDULER_CTX } as *mut SchedulerCtx;
    unsafe { sched_ctx_restore(ctx) };
}
