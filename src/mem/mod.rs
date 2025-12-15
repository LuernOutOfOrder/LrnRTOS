use core::mem;

use crate::platform::mem::platform_init_mem;

#[derive(Debug, Copy, Clone)]
pub struct Memory {
    pub addr: usize,
    pub size: usize,
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
        }
    }
}

static mut MEMORY: Memory = Memory::init_default();

pub fn memory_init() {
    let init_mem: Memory = Memory::init();
    #[allow(static_mut_refs)]
    unsafe { MEMORY = init_mem };
}
