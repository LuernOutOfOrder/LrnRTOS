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
    arch::{
        helpers::current_cpu_core,
        scheduler::{sched_ctx_restore, SchedulerCtx, SCHEDULER_CTX},
    }, config::{BLOCK_QUEUE_MAX_SIZE, CPU_CORE_NUMBER, RUN_QUEUE_MAX_SIZE, TASK_MAX_PRIORITY}, kprint, log, misc::{clear_reschedule, read_need_reschedule}, primitives::{bitmap::Bitmap, indexed_linked_list::IndexedLinkedList, ring_buff::RingBuffer}, task::{
        list::{task_list_get_idle_task, task_list_get_task_by_pid, task_list_update_task_by_pid}, task_awake_block_control, task_awake_tick, task_context_switch, task_pid, task_priority, TaskState, TASK_HANDLER
    }, LogLevel
};

// Reflect the run queue state
// Array of bitmaps, one bitmap per CPU core
pub static mut RUN_QUEUE_BITMAP: [Bitmap; CPU_CORE_NUMBER] =
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
pub static mut BLOCKED_QUEUE: [IndexedLinkedList<BLOCK_QUEUE_MAX_SIZE>; CPU_CORE_NUMBER] =
    [const { IndexedLinkedList::new() }; CPU_CORE_NUMBER];

/// Temporary function use to test the context switch and context restore on multiple task.
/// Will certainly be used later on the real scheduler.
/// Pop oldest task from RingBuffer, save the task context, update it, and repush it to the
/// RingBuffer.
/// Read on the RingBuffer to get the next task, update it, and update the RingBuffer.
/// Not the best way to use the RingBuffer but it will do.
#[unsafe(no_mangle)]
pub fn scheduler() {
    let core: usize = current_cpu_core();
    #[allow(static_mut_refs)]
    let current_run_queue = &mut unsafe { RUN_QUEUE }[core];
    #[allow(static_mut_refs)]
    let current_blocked_queue = &mut unsafe { BLOCKED_QUEUE }[core];
    let current_run_queue_bitmap = &mut unsafe { RUN_QUEUE_BITMAP }[core];
    // Check the need_reschedule flag
    // If a resched has been trigger, pop the head of the blocked queue, update the task and push
    // it to the run queue.
    // Don't check the awake tick or anything else, we consider that if the need_resched flag is
    // true, then the task is available to wake up.
    let resched = read_need_reschedule();
    if resched {
        log!(
            LogLevel::Debug,
            "Reschedule needed, updating queues, clearing the need reschedule bit."
        );
        // Pop from blocked queue and move the task to the run queue
        let wake_up_task = current_blocked_queue.pop();
        let pid: u16;
        if wake_up_task.is_none() {
            log!(
                LogLevel::Error,
                "Error getting the wake up task from blocked queue, blocked queue or need_reschedule flag can be corrupted."
            );
            // Trigger a context switch on current task to avoid to fail-fast
            // TODO:
            return;
        } else {
            // Allow unwrap, we check the value before
            pid = wake_up_task.unwrap().id as u16;
        }
        // Consider the `pid` as init, if wake_up_task.is_none(), we switch on the current task, so
        // we cannot reach this point unless wake_up_task is some and `pid` is set.
        let mut task = task_list_get_task_by_pid(pid).expect("Failed to get the task by it's pid.");
        let priority: u8 = task_priority(&task);
        task_awake_block_control(task);
        task.state = TaskState::Ready;
        task_list_update_task_by_pid(pid, *task);
        current_run_queue[priority as usize].push(pid);
        current_run_queue_bitmap.set_bit(priority as usize);
        clear_reschedule();
    }
    // Current running task
    let mut current_task = unsafe { *TASK_HANDLER };
    if current_task.state == TaskState::Blocked {
        let pid = task_pid(&current_task);
        let priority = task_priority(&current_task);
        let awake_tick = task_awake_tick(&current_task).expect("Failed to get the task awake_tick");
        // Push the current task to the blocked queue
        current_blocked_queue.push(pid as usize, awake_tick);
        // Check the run queue from the current_task priority.
        // If the run queue is empty, clean the run queue bitmap for this priority bit.
        let is_run_queue_empty = current_run_queue[priority as usize].size();
        if is_run_queue_empty == 0 {
            current_run_queue_bitmap.clear_bit(priority as usize);
        }
    }
    if current_task.state != TaskState::Blocked {
        current_task.state = TaskState::Ready;
        let pid = task_pid(&current_task);
        let priority: usize = task_priority(&current_task).into();
        task_list_update_task_by_pid(pid, current_task);
        // Push current task to the priority buffer
        current_run_queue[priority as usize].push(pid);
        // Update the bitmap priority bit.
        current_run_queue_bitmap.set_bit(priority as usize);
    }

    // Update and load next task
    #[allow(static_mut_refs)]
    let is_no_task = current_run_queue_bitmap.is_bitmap_zero();
    if is_no_task {
        log!(
            LogLevel::Debug,
            "No task available in the run queue, enter idle task."
        );
        let idle = task_list_get_idle_task();
        #[allow(clippy::expect_used)]
        task_context_switch(idle.expect("ERROR: failed to get the idle task, invariant violated."));
    }
    let highest_priority: usize = current_run_queue_bitmap.find_leading_bit();
    let get_next_task = current_run_queue[highest_priority].pop();
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
