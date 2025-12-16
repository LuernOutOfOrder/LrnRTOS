mod kernel;

use core::mem;

use kernel::{KernelStack, KERNEL_STACK};

use crate::{config::KERNEL_STACK_SIZE, platform::mem::platform_init_mem};

#[derive(Debug, Copy, Clone)]
pub struct Memory {
    pub addr: usize,
    pub size: usize,
    pub ram_start: usize,
    pub ram_end: usize,
}

impl Memory {
    pub const fn init_default() -> Self {
        unsafe { mem::zeroed() }
    }
    pub fn init() -> Self {
        let platform_mem = platform_init_mem();
        Memory {
            addr: platform_mem.reg.addr,
            size: platform_mem.reg.size,
            ram_start: platform_mem.reg.addr,
            ram_end: platform_mem.reg.addr + platform_mem.reg.size,
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
    let stack_top: usize = unsafe { MEMORY.ram_end };
    let stack_bottom: usize = unsafe { MEMORY.ram_end - KERNEL_STACK_SIZE };
    unsafe {KERNEL_STACK = KernelStack { top: stack_top, bottom: stack_bottom }};
}

fn set_kernel_sp() {

}
