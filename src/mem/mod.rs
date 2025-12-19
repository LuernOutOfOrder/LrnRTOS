mod kernel;

use core::{arch::asm, mem};

use kernel::{KERNEL_STACK, KernelStack};

use crate::{
    arch, config::KERNEL_STACK_SIZE, log, logs::LogLevel, platform::mem::platform_init_mem,
};

pub struct Memory {
    pub mem_start: usize,
    pub mem_end: usize,
}

impl Memory {
    pub const fn init_default() -> Self {
        unsafe { mem::zeroed() }
    }
    pub fn init() -> Self {
        let platform_mem = platform_init_mem();
        Memory {
            mem_start: platform_mem.reg.addr,
            mem_end: platform_mem.reg.addr + platform_mem.reg.size,
        }
    }
}

static mut MEMORY: Memory = Memory::init_default();

/// Init memory from platform
pub fn memory_init() {
    let init_mem: Memory = Memory::init();
    #[allow(static_mut_refs)]
    unsafe {
        MEMORY = init_mem
    };
    let stack_top: usize = unsafe { MEMORY.mem_end };
    let stack_bottom: usize = unsafe { MEMORY.mem_end - KERNEL_STACK_SIZE };
    unsafe {
        KERNEL_STACK = KernelStack {
            top: stack_top,
            bottom: stack_bottom,
        }
    };
    log!(LogLevel::Debug, "Switch to new kernel stack...");
    update_kernel_sp();
}

#[unsafe(no_mangle)]
fn update_kernel_sp() {
    unsafe { asm!("mv a0, {}", in(reg) KERNEL_STACK.top) };
    unsafe { arch::asm::set_kernel_sp() };
}
