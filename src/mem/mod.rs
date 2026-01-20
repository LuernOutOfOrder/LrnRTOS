/*
File info: Kernel memory management.

Test coverage: All basic implementation and check correct value compared to Qemu memory.

Tested:
- Memory structure methods.

Not tested:
- The switch from the early boot stack, and final kernel stack.

Reasons:
- Hard to unit test, so just need to check the invariant during the test flow to see if the stack is correctly updated.

Tests files:
- 'src/tests/mem/mod.rs'
*/

mod kernel;

use core::{arch::asm, mem};

use kernel::{__kernel_end, __kernel_start, KernelStack};

use crate::{arch, config::KERNEL_STACK_SIZE, platform::mem::platform_init_mem};

pub struct Memory {
    // Final kernel stack
    pub kernel_stack: KernelStack,
    // Mem reg of the kernel image(section .text, .data, .bss, etc.)
    pub kernel_img_start: usize,
    pub kernel_img_end: usize,
    // Low addr
    pub mem_start: usize,
    // hi addr
    pub mem_end: usize,
}

impl Memory {
    pub const fn init_default() -> Self {
        unsafe { mem::zeroed() }
    }
    pub fn init() -> Self {
        let platform_mem = platform_init_mem();
        Memory {
            kernel_stack: KernelStack { top: 0, bottom: 0 },
            mem_start: platform_mem.reg.addr,
            mem_end: platform_mem.reg.addr + platform_mem.reg.size,
            kernel_img_start: unsafe { &__kernel_start } as *const u8 as usize,
            kernel_img_end: unsafe { &__kernel_end } as *const u8 as usize,
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
        MEMORY.kernel_stack = KernelStack {
            top: stack_top,
            bottom: stack_bottom,
        }
    };
}

#[unsafe(no_mangle)]
pub fn update_kernel_sp() {
    unsafe { asm!("mv a0, {}", in(reg) MEMORY.kernel_stack.top) };
    unsafe { arch::asm::set_kernel_sp() };
}

pub fn mem_kernel_stack_info<'a>() -> &'a KernelStack {
    #[allow(static_mut_refs)]
    unsafe {
        &MEMORY.kernel_stack
    }
}
