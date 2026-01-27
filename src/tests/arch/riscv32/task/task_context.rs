use crate::{
    arch::task::task_context::TaskContext,
    test_failed,
    tests::{TEST_MANAGER, TestBehavior, TestCase, TestSuite, TestSuiteBehavior},
};

/// Test purpose function to create a task context.
/// DO NOT USE OUTSIDE OF THOSE TESTS
fn test_task_context_entry_ptn_fn() -> ! {
    #[allow(clippy::empty_loop)]
    loop {}
}

pub fn test_task_context_init() -> u8 {
    let task_size: [usize; 2] = [0x8800_0000, 0x8700_0000];
    let task_context: TaskContext = TaskContext::init(task_size, test_task_context_entry_ptn_fn);
    // Check context field that are testable
    // Check GPRs
    for i in 0..32 {
        if task_context.gpr[i] != 0 {
            test_failed!(
                "One of the gpr register has not been initialized at 0. This can lead to UB"
            );
            return 1;
        }
    }
    // Check address_space
    if task_context.address_space[0] != task_size[0] as u32
        || task_context.address_space[1] != task_size[1] as u32
    {
        panic!("Task context has been initialized with wrong address space.");
    }
    // Check pc
    if task_context.pc != test_task_context_entry_ptn_fn as usize as u32 {
        panic!(
            "Task context has been initialized with wrong PC, expect pc to be set to the address of the given function"
        );
    }
    // Check sp
    if task_context.sp != task_size[0] as u32 {
        panic!(
            "Task context has been initialized with wrong SP, expect sp to be set to the hi address of the task address space"
        );
    }
    0
}

pub fn task_context_test_suite() {
    const TASK_CONTEXT_TEST_SUITE: TestSuite = TestSuite {
        tests: &[
            TestCase::init(
                "Task context init",
                test_task_context_init,
                TestBehavior::Default,
            ),
            // TestCase::init(
            //     "Trap frame initialization",
            //     test_trap_frame_init,
            //     TestBehavior::Default,
            // ),
        ],
        name: "RISC-V32 bit task context layout",
        tests_nb: 2,
        behavior: TestSuiteBehavior::Default,
    };
    #[allow(static_mut_refs)]
    unsafe {
        TEST_MANAGER.add_suite(&TASK_CONTEXT_TEST_SUITE)
    };
}
