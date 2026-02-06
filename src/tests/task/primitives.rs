use crate::{
    arch::traps::interrupt::enable_and_halt,
    arch::traps::{disable_interrupts, enable_interrupts, trap_frame::init_trap_frame},
    config::TICK_SAFETY_DURATION,
    drivers::timer::TIMER_SUBSYSTEM,
    ktime::{set_ktime_ms, tick::get_tick},
    print,
    task::{
        CURRENT_TASK_PID, TASK_HANDLER,
        list::task_list_get_task_by_pid,
        primitives::{delay, r#yield},
        task_context_switch, task_create,
    },
    test_failed, test_info,
    tests::{TEST_MANAGER, TestBehavior, TestCase, TestSuite, TestSuiteBehavior},
};
use core::ptr;

fn task_fn() -> ! {
    let mut i: usize = 0;
    loop {
        print!("delay: {i}\n");
        delay(1000);
        if i >= 8 {
            // Exit Qemu
            unsafe { ptr::write_volatile(0x100000 as *mut u32, 0x5555) };
        }
        i += 1;
    }
}

#[unsafe(no_mangle)]
fn test_task_primitives_delay() -> u8 {
    task_create("Test delay", task_fn, 1, 0x1000);
    unsafe { CURRENT_TASK_PID = 2 };
    let mut task = task_list_get_task_by_pid(unsafe { CURRENT_TASK_PID });
    unsafe { TASK_HANDLER = *task.as_mut().unwrap() };
    test_info!(
        "The next output should be the task 'Test delay' printing an integer. The final output should be: 'delay: '"
    );
    let sub = TIMER_SUBSYSTEM.get_primary_timer();
    print!("debug: {:?}\n", sub.timer_type);
    // loop {
    //     print!("delay\n");
    //     unsafe { enable_and_halt() };
    // }
    // task_context_switch(task.unwrap());
    0
}

pub fn task_primitives_test_suite() {
    const TASK_PRIMITIVES_TEST_SUITE: TestSuite = TestSuite {
        tests: &[TestCase::init(
            "Task primitive delay",
            test_task_primitives_delay,
            TestBehavior::Default,
        )],
        name: "Task primitives",
        behavior: TestSuiteBehavior::Default,
    };
    #[allow(static_mut_refs)]
    unsafe {
        TEST_MANAGER.add_suite(&TASK_PRIMITIVES_TEST_SUITE)
    };
}
