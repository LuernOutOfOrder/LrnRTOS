pub mod task_context;

// Asm function for task context switch
unsafe extern "C" {
    // Restore the task context
    pub fn restore_context(context: usize);
    // Save the current task context
    pub fn save_context(context: usize);
}
