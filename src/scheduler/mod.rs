use crate::{
    BUFFER, kprint, kprint_fmt, log,
    logs::LogLevel,
    task::{TaskState, task_context_save, task_context_switch},
};

/// Temporary function use to test the context switch and context restore on multiple task.
/// Will certainly be used later on the real scheduler.
/// Pop oldest task from RingBuffer, save the task context, update it, and repush it to the
/// RingBuffer.
/// Read on the RingBuffer to get the next task, update it, and update the RingBuffer.
/// Not the best way to use the RingBuffer but it will do.
#[unsafe(no_mangle)]
pub fn dispatch() {
    // Save and update current task
    #[allow(static_mut_refs)]
    let current_task = unsafe { BUFFER.read() };
    if current_task.is_none() {
        log!(
            LogLevel::Error,
            "Error getting the last task from RingBuffer"
        );
    }
    let task = current_task.unwrap();
    task_context_save(&*task);
    task.state = TaskState::Ready;
    #[allow(static_mut_refs)]
    unsafe {
        BUFFER.update(*task)
    };
    // Update and load next task
    #[allow(static_mut_refs)]
    let new_task = unsafe { BUFFER.read() };
    if new_task.is_none() {
        log!(
            LogLevel::Error,
            "Error getting the last task from RingBuffer"
        );
    }
    let task = new_task.unwrap();
    task.state = TaskState::Running;
    #[allow(static_mut_refs)]
    unsafe {
        BUFFER.update(*task)
    };
    task_context_switch(task);
}
