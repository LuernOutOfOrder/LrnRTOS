/*
File info: Task primitives.

Test coverage: yield and sleep.

Tested:
- yield with two task.
- sleep with invariants from run queue and blocked queue.

Not tested:
- delay

Reasons:
- delay is hard to test, for now we test it by just checking it manually.

Tests files:
- 'src/tests/task/primitives.rs'
*/

use crate::{
    arch::{helpers::current_cpu_core, traps::interrupt::enable_and_halt},
    kprint, kprint_fmt,
    ktime::{set_ktime_ms, tick::get_tick},
    log,
    logs::LogLevel,
    misc::need_reschedule,
    scheduler::{BLOCKED_QUEUE, RUN_QUEUE, scheduler},
};

use super::{
    TASK_HANDLER, Task, TaskBlockControl, TaskState,
    list::{task_list_get_task_by_pid, task_list_update_task_by_pid},
    task_pid,
};

unsafe extern "C" {
    // Put the current task to sleep until the number of tick given is passed
    // tick: the number of tick the task need to sleep.
    pub fn sleep(tick: usize);
    // Yield function for cooperative scheduling
    pub fn r#yield();
}

// Use no mangle because this function is called from an asm function
// Called from sleep primitive
#[unsafe(no_mangle)]
fn task_set_wake_tick(tick: usize) {
    let current_tick = get_tick();
    let awake_tick = current_tick + tick;
    // Call task primitive to update current task state
    task_block_until(awake_tick);
    // Call a re-schedule
    scheduler();
}

/// Block the current task until the given tick is reach.
/// Update the current task to block it, but the task is still in the run queue, it'll be remove
/// from the run queue and saved in the blocked queue in the scheduler.
pub fn task_block_until(tick: usize) {
    let current_task: *mut Task = unsafe { TASK_HANDLER };
    if current_task.is_null() {
        log!(
            LogLevel::Error,
            "Error getting the current task, invariant violated. Sleep couldn't be used outside of a task."
        );
        // See how to handle this, what to return or something else.
    }
    // Update task
    // Update current state to block
    // Update block control and pass the awake_tick to it
    unsafe {
        // Deref and cast current_task to &mut to update the Task behind the ptr.
        let task: &mut Task = &mut *current_task;
        task.state = TaskState::Blocked;
        task.block_control = TaskBlockControl::AwakeTick(tick);
    }
}

/// Check all the blocked queue to find the task to awake. Just update the task that need to be
/// awake, make them ready. Don't handle the queue by itself.
/// TODO: Use a better data structure than a RingBuffer for the blocked queue.
pub fn task_awake_blocked(tick: usize) {
    // Current CPU core
    let core: usize = current_cpu_core();
    // Current blocked queue
    let current_blocked_queue = unsafe { BLOCKED_QUEUE }[core];
    #[allow(static_mut_refs)]
    let size = current_blocked_queue.get_count();
    if size == 0 {
        return;
    }
    #[allow(static_mut_refs)]
    let mut blocked_task = current_blocked_queue.get_head_node();
    let mut pid: u16;
    if blocked_task.is_none() {
        log!(
            LogLevel::Error,
            "Error getting the oldest task in run queue"
        );
        return;
    } else {
        // Allow unwrap, we check the value before
        pid = blocked_task.unwrap().id as u16;
    }
    // Allow expect, check the value before and if the pid become invalid we don't want to pursue
    // run time.
    #[allow(clippy::expect_used)]
    let task = task_list_get_task_by_pid(pid);
    if task.is_none() {
        log!(
            LogLevel::Error,
            "Error getting the task by pid, the task may not exist"
        );
        return;
    }
    // Allow expected, we check the value before, if it's some, there's shouldn't be any problem by
    // unwrapping it.
    // TODO: just need to correctly used the blocked queue to avoid getting the task from the
    // task_list with pid, and matching on the block_control of the task to awake it.
    #[allow(clippy::expect_used)]
    match task
        .expect("Failed to get the task behind the Option<>. This shouldn't be possible")
        .block_control
    {
        TaskBlockControl::AwakeTick(awake_tick) => {
            if tick >= awake_tick {
                // push to run queue
                #[allow(static_mut_refs)]
                unsafe {
                    // Set the need reschedule flag, the scheduler will check the block queue to
                    // awake correctly the task.
                    need_reschedule();
                };
            } else {
                return;
            }
        }
        TaskBlockControl::None => (),
    }
}

/// Interrupt all operation on the CPU for the given time.
pub fn delay(ms: usize) {
    set_ktime_ms(ms as u64);
    unsafe { enable_and_halt() };
}
