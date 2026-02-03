use crate::{
    task::{list::task_list_size, task_create},
    test_info,
    tests::{TEST_MANAGER, TestBehavior, TestCase, TestSuite, TestSuiteBehavior},
};

pub mod list;

/// This function is only used to create task for testing purpose.
/// This must never be used in other cases
fn task_fn_ptr() -> ! {
    #[allow(clippy::empty_loop)]
    loop {}
}

pub fn test_task_create() -> u8 {
    let list_size = task_list_size();
    if list_size != 0 {
        panic!("Task list should be initialized at 0.");
    }
    test_info!("The next output should be: Successfully created task: Testing task");
    task_create("Testing task", task_fn_ptr, 7, 64);
    let update_list_size = task_list_size();
    if update_list_size != 1 {
        panic!("Task list should have been updated with the created task.");
    }
    0
}

pub fn task_test_suite() {
    const TASK_TEST_SUITE: TestSuite = TestSuite {
        tests: &[TestCase::init(
            "Task creation and register to task list",
            test_task_create,
            TestBehavior::Default,
        )],
        name: "Task",
        behavior: TestSuiteBehavior::Default,
    };
    #[allow(static_mut_refs)]
    unsafe {
        TEST_MANAGER.add_suite(&TASK_TEST_SUITE)
    };
}
