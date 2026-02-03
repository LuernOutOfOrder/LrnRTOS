/*
File info: Kernel memory management.

Test coverage: All basic implementation and check correct value compared to Qemu memory.

Tested:
- Memory structure methods.
- Task allocation.

Not tested:
- The switch from the early boot stack, and final kernel stack.

Reasons:
- Hard to unit test, so just need to check the invariant during the test flow to see if the stack is correctly updated.

Tests files:
- 'src/tests/mem/mod.rs'
*/

mod kernel;

use core::mem;

use kernel::{__kernel_end, __kernel_start, KernelStack};

use crate::{
    arch::mem::update_kernel_sp, config::KERNEL_STACK_SIZE, log, logs::LogLevel,
    platform::mem::platform_init_mem,
};

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

    // Allow unused because this method can be useful for later
    #[allow(unused)]
    fn mem_available(&self) -> [usize; 2] {
        [self.available, self.kernel_img_end]
    }

    pub fn task_alloc(&mut self, size: usize) -> Option<[usize; 2]> {
        let available = self.available;
        let bottom = self.kernel_img_end;
        if available <= size {
            log!(
                LogLevel::Error,
                "Error allocating new task stask. No available space, try reducing the task stack size"
            );
            return None;
        }
        // Compute new available address from available - size asked.
        let check = available - size;
        // Align check on 16 bytes under check
        let check_align = check & !(16 - 1);
        // The size asked must pass between available and bottom
        if check_align > bottom {
            // Update available to exclude new allocated region and subtract 4 bytes to avoid any
            // overlap
            self.available = check_align;
            // Return memory region usable by the new task.
            return Some([available, check_align]);
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
    let stack_top_aligned: usize = stack_top & !(16 - 1);
    if stack_top_aligned <= KERNEL_STACK_SIZE {
        panic!("Failed to initialize memory, try to reduce KERNEL_STACK_SIZE");
    }
    let stack_bottom: usize = unsafe { MEMORY.mem_end - KERNEL_STACK_SIZE };
    // Align stack bottom on 16 bytes under stack bottom address.
    let stack_bottom_aligned: usize = stack_bottom & !(16 - 1);
    if stack_bottom_aligned <= unsafe { MEMORY.kernel_img_end } {
        panic!(
            "Failed to initialize memory, no available space for KERNEL_STACK_SIZE, try reducing it."
        )
    }
    // One word below kernel stack
    let available: usize = stack_bottom_aligned;
    // Check the delta between stack_bottom and available.
    // If different from the size of a usize, panic.
    // Avoid running the kernel if the memory allocation is not stable.
    if stack_bottom - available != 0 {
        panic!(
            "Computation of available address is wrong. Kernel cannot run if memory allocation is unstable"
        );
    }
    // Update MEMORY with new kernel stack
    unsafe {
        MEMORY.kernel_stack = KernelStack {
            top: stack_top_aligned,
            bottom: stack_bottom_aligned,
        }
    };
    // Update MEMORY with address usable for future task
    unsafe {
        MEMORY.available = available;
    }
}

pub fn mem_kernel_stack_info<'a>() -> &'a KernelStack {
    #[allow(static_mut_refs)]
    unsafe {
        &MEMORY.kernel_stack
    }
}

pub fn mem_update_kernel_sp() {
    let sp: usize = unsafe { MEMORY.kernel_stack.top };
    update_kernel_sp(sp);
}

/// Return the hi and lo address of the RAM
/// first index is hi, second is lo
pub fn mem_reg_info() -> [usize; 2] {
    let hi = unsafe { MEMORY.mem_end };
    let lo = unsafe { MEMORY.mem_start };
    [hi, lo]
}

/// Return hi and lo address usable.
/// First element of array is the hi address usable, last one is lo address usable.
pub fn mem_task_alloc(size: usize) -> Option<[usize; 2]> {
    // Allow static mut refs for now
    // TODO: improve memory static to not use mut if possible
    #[allow(static_mut_refs)]
    unsafe {
        MEMORY.task_alloc(size)
    }
}
