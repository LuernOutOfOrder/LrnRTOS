use crate::tests::{TEST_MANAGER, TestSuite, TestSuiteBehavior};

pub fn task_list_test_suite() {
    const TASK_LIST_TEST_SUITE: TestSuite = TestSuite {
        tests: &[],
        name: "Task list",
        tests_nb: 0,
        behavior: TestSuiteBehavior::Skipped,
    };
    #[allow(static_mut_refs)]
    unsafe {
        TEST_MANAGER.add_suite(&TASK_LIST_TEST_SUITE)
    };
}
