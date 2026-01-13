use crate::{
    arch::traps::{handler::trap_handler, trap_frame::TrapFrame},
    ktime::tick::get_tick,
    tests::{TEST_MANAGER, TestBehavior, TestCase, TestSuite},
};

pub fn test_trap_handler_timer_interrupt() {
    let cause: usize = 2147483655;
    // Random mepc
    // TODO: improve mepc security in trap handler
    let mepc: usize = 125696;
    let current_tick = get_tick();
    let mut trap_frame = TrapFrame::init();
    unsafe { trap_handler(mepc, 0, cause, 0, 0, &mut trap_frame) };
    let update_tick = get_tick();
    if current_tick == update_tick {
        panic!(
            "Timer interrupt not correctly handled. Global tick should be updated at each timer interrupt handled"
        );
    }
}

pub fn trap_handler_test_suite() {
    const TRAP_HANDLER_TEST_SUITE: TestSuite<'_> = TestSuite {
        tests: &[TestCase::init(
            "RISC-V 32 bits trap handler timer interrupt",
            test_trap_handler_timer_interrupt,
            TestBehavior::Default,
        )],
        name: "Trap handler",
        tests_nb: 1,
    };
    #[allow(static_mut_refs)]
    unsafe {
        TEST_MANAGER.add_suite(&TRAP_HANDLER_TEST_SUITE)
    };
}
