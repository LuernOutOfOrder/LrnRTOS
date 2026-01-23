use core::arch::asm;

use crate::task::{Task, TaskState};

use super::{new_task_context, restore_context};

#[repr(C)]
pub struct TaskContext {
    pub gpr: [u32; 32],           // Offset 0
    pub address_space: [u32; 2],  // Offset 128 (first index 128; second index 132)
    pub pc: u32,                  // Offset 136
    pub sp: u32,                  // Offset 140
    pub flags: [u8; 3],           // Offset 144 (first index 144; second index 145, third index 146)
    pub instruction_register: u8, // Offset 147
}

impl TaskContext {
    pub const fn init(size: [usize; 2]) -> Self {
        TaskContext {
            gpr: [0u32; 32],
            address_space: [size[0] as u32, size[1] as u32],
            pc: 0,
            sp: 0,
            flags: [0u8; 3],
            instruction_register: 0,
        }
    }

    pub fn new_context_switch(&self, task_func: fn() -> !) {
        // Ptr to self struct
        let self_ptr = self as *const _ as usize;
        unsafe { asm!("mv s0, {}", in(reg) self_ptr) };
        // Set task entry point to s2 reg
        let task_entry_ptr = task_func;
        unsafe { asm!("mv s1, {}", in(reg) task_entry_ptr) };
        // Call new_task_context asm fn
        unsafe { new_task_context() };
    }

    pub fn context_switch(&self) {
        // Save the ptr to self struct to s1 reg, use saved registers to preserved it from across
        // call 
        let self_ptr = self as *const _ as usize;
        unsafe { asm!("mv s0, {}", in(reg) self_ptr) };
        unsafe { restore_context() };
    }
}
