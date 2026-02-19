use crate::{
    RUN_QUEUE_BITMAP,
    arch::{
        helpers::current_cpu_core,
        traps::{
            disable_interrupts, enable_interrupts,
            handler::trap_handler,
            interrupt::enable_and_halt,
            trap_frame::{TrapFrame, init_trap_frame},
        },
    },
    config::TICK_SAFETY_DURATION,
    kprint,
    ktime::{set_ktime_ms, set_ktime_seconds, tick::get_tick},
    scheduler::{BLOCKED_QUEUE, RUN_QUEUE},
    task::{
        CURRENT_TASK_PID, TASK_HANDLER,
        list::task_list_get_task_by_pid,
        primitives::{delay, sleep, r#yield},
        task_context_switch, task_create,
    },
    test_failed, test_info,
    tests::{TEST_MANAGER, TestBehavior, TestCase, TestSuite, TestSuiteBehavior},
};
use core::ptr;

fn task_fn() -> ! {
    let mut i: usize = 0;
    loop {
        kprint!("delay\n");
        delay(1000);
        if i >= 8 {
            // Exit Qemu
            unsafe { ptr::write_volatile(0x100000 as *mut u32, 0x5555) };
        }
        i += 1;
    }
}

fn task_sleep_fn() -> ! {
    loop {
        unsafe {
            sleep(2);
        }
    }
}

fn task_testing_sleep() -> ! {
    let cause: usize = 2147483655;
    // Random mepc
    // TODO: improve mepc security in trap handler
    let mepc: usize = 125696;
    let current_tick = get_tick();
    let mut trap_frame = TrapFrame::init();
    unsafe { trap_handler(mepc, 0, cause, 0, 0, &mut trap_frame) };
    let core: usize = current_cpu_core();
    // Blocked queue should have been updated, checking it
    #[allow(static_mut_refs)]
    let current_blocked_queue = unsafe { &mut BLOCKED_QUEUE[core] };
    #[allow(static_mut_refs)]
    let current_run_queue = unsafe { &mut RUN_QUEUE[core] };
    if current_run_queue[1].size() != 0 {
        test_failed!(
            "The run queue should be empty, got: {}",
            current_run_queue[1].size()
        );
        // Use infinite loop to make the CI crash from timeout. Can't return test failed from
        // here.
        loop {}
    }
    if current_blocked_queue.get_count() != 1 {
        test_failed!(
            "The block queue should have 1 task in it, got: {}",
            current_blocked_queue.get_count()
        );
        // Use infinite loop to make the CI crash from timeout. Can't return test failed from
        // here.
        loop {}
    }
    unsafe { trap_handler(mepc, 0, cause, 0, 0, &mut trap_frame) };
    if current_blocked_queue.get_count() != 0 {
        test_failed!(
            "The block queue should be empty, got: {}",
            current_blocked_queue.get_count()
        );
        // Use infinite loop to make the CI crash from timeout. Can't return test failed from
        // here.
        loop {}
    }
    if current_run_queue[1].size() != 1 {
        test_failed!(
            "The run queue should have 1 task in it, got: {}",
            current_run_queue[1].size()
        );
        // Use infinite loop to make the CI crash from timeout. Can't return test failed from
        // here.
        loop {}
    }
    test_info!("Invariant from sleep and blocked queue successfully respected. Exit qemu...");
    unsafe { ptr::write_volatile(0x100000 as *mut u32, 0x5555) };
    // Check with condition the invariant, blocked queue updated etc
    // Recall trap handler, then check that the block queue is empty.
    loop {}
}

fn test_task_primitives_delay() -> u8 {
    task_create("Test delay", task_fn, 1, 0x1000);
    unsafe { CURRENT_TASK_PID = 2 };
    let mut task = task_list_get_task_by_pid(unsafe { CURRENT_TASK_PID });
    unsafe { TASK_HANDLER = *task.as_mut().unwrap() };
    test_info!(
        "The next output should be the task 'Test delay' printing an integer. The final output should be: 'delay: '"
    );
    set_ktime_seconds(TICK_SAFETY_DURATION);
    enable_interrupts();
    task_context_switch(task.unwrap());
    0
}

fn test_task_primitives_sleep() -> u8 {
    // pid 2
    task_create("Test sleep", task_sleep_fn, 1, 0x1000);
    // pid 3
    task_create("Test sleep invariants", task_testing_sleep, 1, 0x1000);
    unsafe { CURRENT_TASK_PID = 2 };
    let mut task = task_list_get_task_by_pid(unsafe { CURRENT_TASK_PID });
    unsafe { TASK_HANDLER = *task.as_mut().unwrap() };
    #[allow(static_mut_refs)]
    unsafe {
        // Access the queue and bitmap from CPU core 0
        // run queue priority 1
        RUN_QUEUE[0][1].push(3);
        RUN_QUEUE_BITMAP[0].set_bit(1);
    }
    task_context_switch(task.unwrap());
    0
}

pub fn task_primitives_test_suite() {
    const TASK_PRIMITIVES_TEST_SUITE: TestSuite = TestSuite {
        tests: &[
            TestCase::init(
                "Task primitive delay",
                test_task_primitives_delay,
                TestBehavior::Skipped,
            ),
            TestCase::init(
                "Task primitive sleep",
                test_task_primitives_sleep,
                TestBehavior::Skipped,
            ),
        ],
        name: "Task primitives",
        behavior: TestSuiteBehavior::Default,
    };
    #[allow(static_mut_refs)]
    unsafe {
        TEST_MANAGER.add_suite(&TASK_PRIMITIVES_TEST_SUITE)
    };
}
