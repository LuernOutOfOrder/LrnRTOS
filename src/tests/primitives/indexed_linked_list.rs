use crate::{
    kprint_fmt,
    primitives::indexed_linked_list::IndexedLinkedList,
    test_failed, test_info,
    tests::{TEST_MANAGER, TestBehavior, TestCase, TestSuite, TestSuiteBehavior},
};

fn test_delta_list_push() -> u8 {
    let mut list: IndexedLinkedList<10> = IndexedLinkedList::new();
    // Push some task
    list.push(1, 70);
    list.push(2, 80);
    list.push(3, 75);
    list.push(4, 50);
    // Get head and tail task
    let head = list.get_head();
    let tail = list.get_tail();
    let head_node = list.get_index(head);
    let tail_node = list.get_index(tail);
    if head_node.id != 4 {
        test_failed!("head node should be the task 4, got: {}\n", head_node.id);
        return 1;
    }
    if head_node.next_node.unwrap() != 0 {
        test_failed!(
            "head node.next_node should be the task 1 at index 0, got: {}\n",
            head_node.next_node.unwrap()
        );
        return 1;
    }
    if tail_node.id != 2 {
        test_failed!("tail node should be the task 2, got: {}\n", tail_node.id);
        return 1;
    }
    if tail_node.next_node.is_some() {
        test_failed!(
            "tail node should not have a next ask, got: {}\n",
            tail_node.next_node.unwrap()
        );
        return 1;
    }
    // Get number of node in the list
    let count = list.get_count();
    if count != 4 {
        test_failed!("count should be 4, got: {}\n", count);
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
