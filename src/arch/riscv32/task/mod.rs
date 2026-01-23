use core::arch::global_asm;

pub mod task_context;

global_asm!(include_str!("context_offset.S"));
global_asm!(include_str!("restore_context.S"));

// Asm function for task context switch
unsafe extern "C" {
    // Restore the task context
    pub fn restore_context(context: usize);
    // Restore context for a newly created task
    pub fn new_task_context(context: usize, entry_point: usize) -> !;
}
