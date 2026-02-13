use crate::{
    kprint_fmt,
    primitives::indexed_linked_list::IndexedLinkedList,
    test_failed, test_info,
    tests::{TEST_MANAGER, TestBehavior, TestCase, TestSuite, TestSuiteBehavior},
};

fn test_delta_list_push() -> u8 {
    let mut list: IndexedLinkedList<10> = IndexedLinkedList::new();
    // Push a short task
    list.push(1, 70);
    list.push(2, 80);
    list.push(3, 75);
    let first_node = list.get_index(0);
    if first_node.id != 1 {
        test_failed!("first node should be the task 1, got: {}\n", first_node.id);
        return 1;
    }
    if first_node.next_node.unwrap() != 2 {
        test_failed!(
            "first node.next_node should be the task 3 at index 2, got: {}\n",
            first_node.next_node.unwrap()
        );
        return 1;
    }
    0
}

pub fn delta_list_primitive_test_suite() {
    const DELTA_LIST_TEST_SUITE: TestSuite = TestSuite {
        tests: &[TestCase::init(
            "IndexedLinkedList push",
            test_delta_list_push,
            TestBehavior::Default,
        )],
        name: "IndexedLinkedList primitive type",
        behavior: TestSuiteBehavior::Default,
    };
    #[allow(static_mut_refs)]
    unsafe {
        TEST_MANAGER.add_suite(&DELTA_LIST_TEST_SUITE)
    };
}
