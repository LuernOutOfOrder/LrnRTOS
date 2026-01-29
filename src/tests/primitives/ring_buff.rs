use crate::{
    primitives::ring_buff::RingBuffer,
    test_failed, test_info,
    tests::{TEST_MANAGER, TestBehavior, TestCase, TestSuite, TestSuiteBehavior},
};

pub fn test_ringbuffer_init() -> u8 {
    let mut ring_buff: RingBuffer<usize, 3> = RingBuffer::init();
    if ring_buff.head() != 0 {
        test_failed!(
            "Ring buffer head should be initialized at 0, got: {}",
            ring_buff.head()
        );
        return 1;
    }
    if ring_buff.tail() != 0 {
        test_failed!(
            "Ring buffer tail should be initialized at 0, got: {}",
            ring_buff.tail()
        );
        return 1;
    }
    test_info!("Next output expected to be: Ring buffer is empty, abort pop.");
    ring_buff.pop();
    ring_buff.push(1);
    ring_buff.push(1);
    test_info!("Next output expected to be: Ring buffer full, abort push.");
    ring_buff.push(1);
    if ring_buff.size() != 2 {
        test_failed!("Ring buffer expected size: 2, got: {}", ring_buff.size());
        return 1;
    }
    0
}

pub fn test_ringbuffer_push() -> u8 {
    let mut ring_buff: RingBuffer<usize, 3> = RingBuffer::init();
    ring_buff.push(1);
    if ring_buff.tail() != 1 && ring_buff.head() != 0 {
        test_failed!(
            "Ring buffer head and tail should been updated correctly when push() is used, expected head: 0\ttail: 1, got: head: {}\ttail: {}",
            ring_buff.head(),
            ring_buff.tail()
        );
        return 1;
    }
    // Unwrap and deref because the test should pass and the value should be 1.
    if ring_buff.read().unwrap() != 1 {
        test_failed!(
            "Pushed value should be: 1, got: {}",
            ring_buff.read().unwrap()
        );
        return 1;
    }
    ring_buff.push(8);
    if ring_buff.tail() != 2 && ring_buff.head() != 0 {
        test_failed!(
            "Ring buffer head and tail should been updated correctly when push() is used, expected head: 0\ttail: 2, got: head: {}\ttail: {}",
            ring_buff.head(),
            ring_buff.tail()
        );
        return 1;
    }
    // Unwrap and deref because the test should pass and the value should be 1.
    if ring_buff.read().unwrap() != 1 {
        test_failed!(
            "Pushed value should be: 1, got: {}",
            ring_buff.read().unwrap()
        );
        return 1;
    }
    test_info!("Next output expected to be: Ring buffer full, abort push.");
    ring_buff.push(12);
    if ring_buff.tail() != 2 && ring_buff.head() != 0 {
        test_failed!(
            "Ring buffer head and tail shouldn't have been updated after handling a possible overflow. Expected head: 0\ttail: 2, got: head: {}\ttail: {}",
            ring_buff.head(),
            ring_buff.tail()
        );
        return 1;
    }
    0
}

pub fn test_ringbuffer_pop() -> u8 {
    let mut ring_buff: RingBuffer<usize, 3> = RingBuffer::init();
    ring_buff.push(1);
    ring_buff.push(16);
    let oldest = ring_buff.pop();
    if oldest.is_none() {
        test_failed!("Ring buffer pop() should not return none");
        return 1;
    }
    if oldest.unwrap() != 1 {
        test_failed!(
            "Ring buffer oldest element should be: 1, got: {}",
            oldest.unwrap()
        );
        return 1;
    }
    if ring_buff.head() != 1 && ring_buff.tail() != 2 {
        test_failed!(
            "Ring buffer head should be 1 and tail should be 2, got head: {}\ttail: {}",
            ring_buff.head(),
            ring_buff.tail()
        );
        return 1;
    }
    let less_oldest_but_still = ring_buff.pop();
    if less_oldest_but_still.is_none() {
        test_failed!("Ring buffer pop() should not return none");
        return 1;
    }
    if less_oldest_but_still.unwrap() != 16 {
        test_failed!(
            "Ring buffer oldest element should be: 16, got: {}",
            less_oldest_but_still.unwrap()
        );
        return 1;
    }
    if ring_buff.head() != 2 && ring_buff.tail() != 2 {
        test_failed!(
            "Ring buffer head should be 2 and tail should be 2, got head: {}\ttail: {}",
            ring_buff.head(),
            ring_buff.tail()
        );
        return 1;
    }
    ring_buff.push(22);
    if ring_buff.head() != 2 && ring_buff.tail() != 0 {
        test_failed!(
            "Ring buffer head should be 2 and tail should be 0, got head: {}\ttail: {}",
            ring_buff.head(),
            ring_buff.tail()
        );
        return 1;
    }
    0
}

pub fn primitives_test_suite() {
    const PRIMITIVES_TEST_SUITE: TestSuite = TestSuite {
        tests: &[
            TestCase::init(
                "RingBuffer initialization",
                test_ringbuffer_init,
                TestBehavior::Default,
            ),
            TestCase::init(
                "RingBuffer push",
                test_ringbuffer_push,
                TestBehavior::Default,
            ),
            TestCase::init("RingBuffer pop", test_ringbuffer_pop, TestBehavior::Default),
        ],
        name: "Primitives",
        tests_nb: 2,
        behavior: TestSuiteBehavior::Default,
    };
    #[allow(static_mut_refs)]
    unsafe {
        TEST_MANAGER.add_suite(&PRIMITIVES_TEST_SUITE)
    };
}
