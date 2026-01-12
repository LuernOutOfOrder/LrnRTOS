use crate::mem::Memory;

use super::TestCase;

pub fn test_memory_impl() {
    let default_mem = Memory::init_default();
    if default_mem.mem_end != 0 || default_mem.mem_start != 0 {
        panic!("Memory init_default should initialize all field at 0");
    }
    let mem = Memory::init();
    if mem.mem_end != 0x88000000 || mem.mem_start != 0x80000000 {
        panic!("Memory init should initialize all field with correct Qemu memory.");
    }
}

pub static KERNEL_MEMORY_TEST_SUITE: &[TestCase] = &[TestCase {
    name: "Memory basic implementation",
    func: test_memory_impl,
}];
