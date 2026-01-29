use crate::{
    BUFFER, log,
    logs::LogLevel,
    task::{
        CURRENT_TASK_PID, TaskState,
        list::{task_list_get_task_by_pid, task_list_update_task_by_pid},
        task_context_save, task_context_switch,
    },
};

/// Temporary function use to test the context switch and context restore on multiple task.
/// Will certainly be used later on the real scheduler.
/// Pop oldest task from RingBuffer, save the task context, update it, and repush it to the
/// RingBuffer.
/// Read on the RingBuffer to get the next task, update it, and update the RingBuffer.
/// Not the best way to use the RingBuffer but it will do.
pub fn dispatch(ra: usize) {
    // Current running task
    let current_task_pid = unsafe { CURRENT_TASK_PID };
    let current_task = task_list_get_task_by_pid(current_task_pid).unwrap();
    task_context_save(&*current_task, ra);
    current_task.state = TaskState::Ready;
    task_list_update_task_by_pid(current_task_pid, *current_task);
    #[allow(static_mut_refs)]
    unsafe {
        BUFFER.push(current_task_pid)
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
    unsafe { CURRENT_TASK_PID = next_task_pid };
    task_context_switch(next_task);
}
