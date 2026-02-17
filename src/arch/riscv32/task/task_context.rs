/*
File info: RISC-V 32 bit task context.

Test coverage: Everything tested and up-to-date.

Tested:
- init methods,
- offset of structure field

Not tested:

Reasons:

Tests files:
- 'src/tests/arch/riscv32/task/task_context.rs'
*/

use super::{restore_context, save_context};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct TaskContext {
    pub gpr: [u32; 32],           // Offset 0
    pub address_space: [u32; 2],  // Offset 128 (first index 128; second index 132)
    pub pc: u32,                  // Offset 136
    pub sp: u32,                  // Offset 140
    pub ra: u32,                  // Offset 144
    pub mstatus: u32,             // Offset 148
    pub flags: [u8; 3],           // Offset 152 (first index 152; second index 153, third index 154)
    pub instruction_register: u8, // Offset 155
}

impl TaskContext {
    pub fn init(size: [usize; 2], func: fn() -> !) -> Self {
        TaskContext {
            gpr: [0u32; 32],
            address_space: [size[0] as u32, size[1] as u32],
            pc: func as usize as u32,
            sp: size[0] as u32,
            ra: func as usize as u32,
            // Set mstatus to 8 by default to enable mie
            mstatus: 6152,
            flags: [0u8; 3],
            instruction_register: 0,
        }
    }

    /// Trigger a context switch for a task
    pub fn context_switch(&self) {
        // Save the ptr to self struct, use saved registers to preserved it from across
        let self_ptr = self as *const _ as usize;
        // Call restore_context asm fn
        // Pass argument here instead of using asm! macro to ensure that the ABI is respected
        unsafe { restore_context(self_ptr) };
    }

    /// Trigger a context saving.
    pub fn context_save(&self, ra: usize, sp: usize) {
        // Save the ptr to self struct, use saved registers to preserved it from across
        let self_ptr = self as *const _ as usize;
        // Call restore_context asm fn
        // Pass argument here instead of using asm! macro to ensure that the ABI is respected
        unsafe { save_context(self_ptr, ra, sp) };
    }
}
