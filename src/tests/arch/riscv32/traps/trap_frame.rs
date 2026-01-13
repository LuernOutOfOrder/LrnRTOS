use crate::{
    arch::traps::trap_frame::{KERNEL_TRAP_FRAME, TRAP_STACK_BUFF, TrapFrame, init_trap_frame},
    test_failed,
    tests::{TEST_MANAGER, TestBehavior, TestCase, TestSuite},
};

pub fn test_trap_frame_init_zeroed() -> u8 {
    // Init trap frame using struct method
    let trap_frame: TrapFrame = TrapFrame::init();
    // Check all field
    let mut gp_regs_size: usize = 0;
    for i in 0..trap_frame.gp_regs.len() {
        if trap_frame.gp_regs[i] != 0 {
            gp_regs_size += 1;
        }
    }
    if gp_regs_size == 0 {
        test_failed!("Trap frame gp_regs field should be initialized empty.");
        return 1;
    }
    if trap_frame.satp != 0 {
        test_failed!("Trap frame satp field should be initialized at 0.");
        return 1;
    }
    if trap_frame.hartid != 0 {
        test_failed!("Trap frame hartid field should be initialized at 0.");
        return 1;
    }
    if !trap_frame.trap_stack.is_null() {
        test_failed!("Trap frame trap_stack field should be initialized at null ptr.");
        return 1;
    }
    0
}

pub fn test_trap_frame_init() -> u8 {
    // Init trap frame using init_trap_frame fn
    init_trap_frame();
    // Ok because test env, no concurrency
    #[allow(static_mut_refs)]
    if unsafe { KERNEL_TRAP_FRAME.trap_stack } != unsafe { TRAP_STACK_BUFF.as_mut_ptr() } {
        panic!("Trap frame trap_stack field should be initialized with ptr to TRAP_STACK_BUFF");
    }
    0
}

pub fn trap_frame_test_suite() {
    const TRAP_FRAME_TEST_SUITE: TestSuite = TestSuite {
        tests: &[
            TestCase::init(
                "Trap frame empty initialization",
                test_trap_frame_init_zeroed,
                TestBehavior::Default,
            ),
            TestCase::init(
                "Trap frame initialization",
                test_trap_frame_init,
                TestBehavior::Default,
            ),
        ],
        name: "Trap frame",
        tests_nb: 2,
    };
    #[allow(static_mut_refs)]
    unsafe {
        TEST_MANAGER.add_suite(&TRAP_FRAME_TEST_SUITE)
    };
}
