use core::arch::global_asm;

global_asm! {
    concat!(
    // Asm entry point of the kernel
    include_str!("start.S"),
    // GNU Macros used for trap handling, and context switch
    include_str!("gnu_macro.S"),
    // Update kernel sp after memory init
    include_str!("set_kernel_sp.S"),
    // Scheduler context switch
    include_str!("sched_context.S"),
    // All task context offsets
    include_str!("context_offset.S"),
    // Context switch function, restore/save
    include_str!("save_context.S"),
    include_str!("restore_context.S"),
    // Trap entry
    include_str!("trap_entry.S")
    )
}

unsafe extern "C" {
    pub fn set_kernel_sp();
}
