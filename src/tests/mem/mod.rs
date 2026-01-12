use crate::{mem::Memory, tests::TEST_MANAGER};

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

pub fn memory_test_suite() {
    let kernel_memory_test_suite: &[TestCase] = &[TestCase {
        name: "Memory basic implementation",
        func: test_memory_impl,
    }];
    #[allow(static_mut_refs)]
    unsafe {
        TEST_MANAGER.add_suite(kernel_memory_test_suite)
    };
}
