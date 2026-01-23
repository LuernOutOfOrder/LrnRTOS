use core::arch::global_asm;

pub mod task_context;

global_asm!(include_str!("context_offset.S"));
global_asm!(include_str!("restore_context.S"));
global_asm!(include_str!("save_context.S"));

// Asm function for task context switch
unsafe extern "C" {
    // Restore the task context
    pub fn restore_context(context: usize);
    // Save the current task context
    pub fn save_context(context: usize);
}
