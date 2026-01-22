use core::arch::global_asm;

pub mod task_context;

global_asm!(include_str!("restore_context.S"));

unsafe extern "C" {
    pub fn restore_context();
}
