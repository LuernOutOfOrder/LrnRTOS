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
    LogLevel,
    arch::scheduler::{SCHEDULER_CTX, SchedulerCtx, sched_ctx_restore},
    config::{BLOCK_QUEUE_MAX_SIZE, CPU_CORE_NUMBER, RUN_QUEUE_MAX_SIZE, TASK_MAX_PRIORITY},
    log,
    misc::{clear_reschedule, read_need_reschedule},
    primitives::{bitmap::Bitmap, ring_buff::RingBuffer},
    task::{
        TASK_HANDLER, TaskState,
        list::{task_list_get_idle_task, task_list_get_task_by_pid, task_list_update_task_by_pid},
        task_context_switch, task_pid, task_priority,
    },
};

// Reflect the run queue state
// Array of bitmaps, one bitmap per CPU core
pub static mut RUN_QUEUE_BITMAP: [Bitmap<u32>; CPU_CORE_NUMBER] =
    [const { Bitmap::new() }; CPU_CORE_NUMBER];
// Array of Array of run queue per priority per CPU core.
// Each index of this array is specific a CPU core, index 0 is for CPU core 0, etc.
// Each index of the inside array is another array, each index is a priority. And at each index, there's a ring buffer of all task
// with that priority.
// We use the RUN_QUEUE_BITMAP to easily find the buffer with the highest priority to look into.
pub static mut RUN_QUEUE: [[RingBuffer<u16, RUN_QUEUE_MAX_SIZE>; TASK_MAX_PRIORITY];
    CPU_CORE_NUMBER] = [[const { RingBuffer::init() }; TASK_MAX_PRIORITY]; CPU_CORE_NUMBER];
// Queue containing all blocked task.
// Same data structure as the RUN_QUEUE.
pub static mut BLOCKED_QUEUE: [RingBuffer<u16, BLOCK_QUEUE_MAX_SIZE>; CPU_CORE_NUMBER] =
    [const { RingBuffer::init() }; CPU_CORE_NUMBER];

/// Temporary function use to test the context switch and context restore on multiple task.
/// Will certainly be used later on the real scheduler.
/// Pop oldest task from RingBuffer, save the task context, update it, and repush it to the
/// RingBuffer.
/// Read on the RingBuffer to get the next task, update it, and update the RingBuffer.
/// Not the best way to use the RingBuffer but it will do.
#[unsafe(no_mangle)]
pub fn scheduler(core: usize) {
    #[allow(static_mut_refs)]
    let current_run_queue = unsafe { RUN_QUEUE }[core];
    #[allow(static_mut_refs)]
    let current_blocked_queue = unsafe { BLOCKED_QUEUE }[core];
    // Current running task
    let mut current_task = unsafe { *TASK_HANDLER };
    if current_task.state != TaskState::Blocked {
        current_task.state = TaskState::Ready;
        let pid = task_pid(&current_task);
        let priority = task_priority(&current_task);
        task_list_update_task_by_pid(pid, current_task);
        #[allow(static_mut_refs)]
        unsafe {
            // Push current task to the priority buffer
            RUN_QUEUE[priority].push(pid)
        };
    }
    let resched = read_need_reschedule();
    if resched {
        log!(
            LogLevel::Debug,
            "Reschedule needed, clearing the need reschedule bit."
        );
        clear_reschedule();
    }
    // Update and load next task
    #[allow(static_mut_refs)]
    let get_next_task = unsafe { RUN_QUEUE.pop() };
    if get_next_task.is_none() {
        log!(
            LogLevel::Debug,
            "No task available in the run queue, enter idle task."
        );
        let idle = task_list_get_idle_task();
        #[allow(clippy::expect_used)]
        task_context_switch(idle.expect("ERROR: failed to get the idle task, invariant violated."));
    }
    // Allow unwrap because it's a temporary function
    #[allow(clippy::unwrap_used)]
    let next_task_pid = get_next_task.unwrap();
    // Allow unwrap because it's a temporary function
    #[allow(clippy::unwrap_used)]
    let next_task = task_list_get_task_by_pid(next_task_pid).unwrap();
    next_task.state = TaskState::Running;
    task_list_update_task_by_pid(next_task_pid, *next_task);
    unsafe { TASK_HANDLER = next_task }
    task_context_switch(next_task);
}

pub fn switch_scheduler_ctx() {
    #[allow(static_mut_refs)]
    let ctx = unsafe { &mut SCHEDULER_CTX } as *mut SchedulerCtx;
    unsafe { sched_ctx_restore(ctx) };
}
