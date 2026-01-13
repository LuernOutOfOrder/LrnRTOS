use crate::{
    mem::Memory,
    tests::{TEST_MANAGER, TestBehavior},
};

use super::{TestCase, TestSuite};

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
    const KERNEL_MEMORY_TEST_SUITE: TestSuite = TestSuite {
        tests: &[TestCase::init(
            "Memory basic implementation",
            test_memory_impl,
            TestBehavior::Default,
        )],
        name: "Kernel memory",
        tests_nb: 1,
    };
    #[allow(static_mut_refs)]
    unsafe {
        TEST_MANAGER.add_suite(&KERNEL_MEMORY_TEST_SUITE)
    };
}
