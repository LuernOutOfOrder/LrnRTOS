use crate::{arch::traps::trap_frame::{init_trap_frame, TrapFrame, KERNEL_TRAP_FRAME, TRAP_STACK_BUFF}, tests::TestCase};

pub fn test_trap_frame_init_zeroed() {
    // Init trap frame using struct method
    let trap_frame: TrapFrame = TrapFrame::init();
    // Check all field
    let mut gp_regs_size: usize = 0;
    for i in 0..trap_frame.gp_regs.len() {
        if trap_frame.gp_regs[i] != 0 {
            gp_regs_size += 1;
        }
    }
    if gp_regs_size != 0 {
        panic!("Trap frame gp_regs field should be initialized empty.");
    }
    if trap_frame.satp != 0 {
        panic!("Trap frame satp field should be initialized at 0.");
    }
    if trap_frame.hartid != 0 {
        panic!("Trap frame hartid field should be initialized at 0.");
    }
    if !trap_frame.trap_stack.is_null() {
        panic!("Trap frame trap_stack field should be initialized at null ptr.");
    }
}

pub fn test_trap_frame_init() {
    // Init trap frame using init_trap_frame fn
    init_trap_frame();
    // Ok because test env, no concurrency
    #[allow(static_mut_refs)]
    if unsafe { KERNEL_TRAP_FRAME.trap_stack } != unsafe { TRAP_STACK_BUFF.as_mut_ptr() } {
        panic!("Trap frame trap_stack field should be initialized with ptr to TRAP_STACK_BUFF");
    }
}

pub static TRAP_FRAME_TEST_SUITE: &[TestCase] = &[
    TestCase {
        name: "Trap frame empty initialization",
        func: test_trap_frame_init_zeroed,
    },
    TestCase {
        name: "Trap frame initialization",
        func: test_trap_frame_init,
    },
];
