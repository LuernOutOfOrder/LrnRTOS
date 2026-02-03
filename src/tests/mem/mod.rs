use crate::{
    mem::{mem_kernel_stack_info, mem_reg_info, mem_task_alloc, memory_init},
    tests::{TEST_MANAGER, TestBehavior, TestSuiteBehavior},
};

use super::{TestCase, TestSuite};

pub fn test_memory_impl() -> u8 {
    memory_init();
    let reg = mem_reg_info();
    if reg[0] != 0x88000000 || reg[1] != 0x80000000 {
        panic!("Memory init should initialize all field with correct Qemu memory.");
    }
    0
}

pub fn test_memory_task_alloc() -> u8 {
    let size: usize = 512;
    let allocate_reg: Option<[usize; 2]> = mem_task_alloc(size);
    let kernel_stack = mem_kernel_stack_info();
    let available = kernel_stack.bottom;
    let correct_available: u32 = 0x87ffbffc_u32;
    if allocate_reg.unwrap()[0] != available {
        panic!(
            "Task allocation hi address should be: {:#x}, got: {:#x}\n",
            correct_available, available
        );
    }
    let task_bottom = available - size;
    let correct_task_bottom: u32 = 0x87ffbdfc_u32;
    if allocate_reg.unwrap()[1] != task_bottom {
        panic!(
            "Task allocation lo address should be: {:#x}, got: {:#x}\n",
            correct_task_bottom, task_bottom
        );
    }
    let compute_allocate_size = allocate_reg.unwrap()[0] - allocate_reg.unwrap()[1];
    if compute_allocate_size != size {
        panic!(
            "Task allocation should have allocate the asked size: {:#x}, got: {:#x}",
            compute_allocate_size, size
        );
    }
    0
}

pub fn memory_test_suite() {
    const KERNEL_MEMORY_TEST_SUITE: TestSuite = TestSuite {
        tests: &[
            TestCase::init(
                "Memory basic implementation",
                test_memory_impl,
                TestBehavior::Default,
            ),
            TestCase::init(
                "Memory task allocation",
                test_memory_task_alloc,
                TestBehavior::Default,
            ),
        ],
        name: "Kernel memory",
        behavior: TestSuiteBehavior::Default,
    };
    #[allow(static_mut_refs)]
    unsafe {
        TEST_MANAGER.add_suite(&KERNEL_MEMORY_TEST_SUITE)
    };
}
