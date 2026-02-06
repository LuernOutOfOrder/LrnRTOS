use super::primitives::task_block_until;
use crate::ktime::tick::get_tick;
use crate::scheduler::dispatch;

unsafe extern "C" {
    // Put the current task to sleep until the number of tick given is passed
    // tick: the number of tick the task need to sleep.
    pub fn sleep(tick: usize);
}

// Use no mangle because this function is called from an asm function
#[unsafe(no_mangle)]
fn task_set_wake_tick(tick: usize) {
    let current_tick = get_tick();
    let awake_tick = current_tick + tick;
    // Call task primitive to update current task state
    task_block_until(awake_tick);
    // Call a re-schedule
    dispatch();
}
