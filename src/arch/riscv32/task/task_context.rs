use super::{restore_context, save_context};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct TaskContext {
    pub gpr: [u32; 32],           // Offset 0
    pub address_space: [u32; 2],  // Offset 128 (first index 128; second index 132)
    pub pc: u32,                  // Offset 136
    pub sp: u32,                  // Offset 140
    pub flags: [u8; 3],           // Offset 144 (first index 144; second index 145, third index 146)
    pub instruction_register: u8, // Offset 147
}

impl TaskContext {
    pub fn init(size: [usize; 2], func: fn() -> !) -> Self {
        TaskContext {
            gpr: [0u32; 32],
            address_space: [size[0] as u32, size[1] as u32],
            pc: func as usize as u32,
            sp: size[0] as u32,
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
    pub fn context_save(&self) {
        // Save the ptr to self struct, use saved registers to preserved it from across
        let self_ptr = self as *const _ as usize;
        // Call restore_context asm fn
        // Pass argument here instead of using asm! macro to ensure that the ABI is respected
        unsafe { save_context(self_ptr) };
    }
}
