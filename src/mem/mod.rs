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

use core::{arch::asm, mem, usize};

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
    // Allocation on the RAM happened from hi to lo
    // That's the address of where the RAM is available, going down
    // Consider this address as usable. Addresses above this one is used, below is available
    pub available: usize,
}

impl Memory {
    const fn init_default() -> Self {
        unsafe { mem::zeroed() }
    }
    fn init() -> Self {
        let platform_mem = platform_init_mem();
        Memory {
            kernel_stack: KernelStack { top: 0, bottom: 0 },
            mem_start: platform_mem.reg.addr,
            mem_end: platform_mem.reg.addr + platform_mem.reg.size,
            kernel_img_start: unsafe { &__kernel_start } as *const u8 as usize,
            kernel_img_end: unsafe { &__kernel_end } as *const u8 as usize,
            available: 0,
        }
    }

    fn mem_available(&self) -> [usize; 2] {
        [self.available, self.kernel_img_end]
    }

    /// Return hi and lo address usable.
    /// First element of array is the hi address usable, last one is lo address usable.
    pub fn task_alloc(&mut self, size: usize) -> Option<[usize; 2]> {
        let available = self.available;
        let bottom = self.kernel_img_end;
        // Compute new available address from available - size asked.
        let check = available - size;
        // The size asked must pass between available and bottom
        if check > bottom {
            // Update available to exclude new allocated region and subtract 4 bytes to avoid any
            // overlap
            self.available = check - mem::size_of::<usize>();
            // Return memory region usable by the new task.
            return Some([available, check]);
        }
        None
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
    // One word below kernel stack
    let available: usize = stack_bottom - core::mem::size_of::<usize>();
    // Check the delta between stack_bottom and available.
    // If different from the size of a usize, panic.
    // Avoid running the kernel if the memory allocation is not stable.
    if stack_bottom - available != core::mem::size_of::<usize>() {
        panic!(
            "Computation of available address is wrong. It should be - sizeoff(usize) under kernel stack bottom address. Kernel cannot run if memory allocation is unstable"
        );
    }
    // Update MEMORY with new kernel stack
    unsafe {
        MEMORY.kernel_stack = KernelStack {
            top: stack_top,
            bottom: stack_bottom,
        }
    };
    // Update MEMORY with address usable for future task
    unsafe {
        MEMORY.available = available;
    }
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

pub fn mem_task_alloc(size: usize) -> Option<[usize; 2]> {
    // Allow static mut refs for now
    // TODO: improve memory static to not use mut if possible
    #[allow(static_mut_refs)]
    unsafe {
        MEMORY.task_alloc(size)
    }
}
