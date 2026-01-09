use super::TestCase;

// Those tests sucks

pub fn test_set_ktime_ms() {
    let cpu_freq = 10000000;
    // 1 for only 1ms
    let duration = 1;
    let delta_ticks = cpu_freq as u64 * duration / 1000;
    if delta_ticks != 10000 {
        panic!(
            "Computation of ktime in ms is wrong. Expected: 10000, got: {}",
            delta_ticks
        );
    }
}

pub fn test_set_ktime_ns() {
    let cpu_freq = 10000000;
    // 1 for only 1ns
    let duration = 1;
    let delta_ticks = cpu_freq as u64 * duration / 1_000_000;
    if delta_ticks != 10 {
        panic!(
            "Computation of ktime in ns is wrong. Expected: 10, got: {}",
            delta_ticks
        );
    }
}

pub fn test_set_ktime_seconds() {
    let cpu_freq = 10000000;
    // 1 for only 1ns
    let duration = 1;
    let delta_ticks = cpu_freq as u64 * duration;
    if delta_ticks != 10000000 {
        panic!(
            "Computation of ktime in ms is wrong. Expected: 10000000, got: {}",
            delta_ticks
        );
    }
}

pub static KTIME_TEST_SUITE: &[TestCase] = &[
    TestCase {
        name: "set_ktime_ms",
        func: test_set_ktime_ms,
    },
    TestCase {
        name: "set_ktime_ns",
        func: test_set_ktime_ns,
    },
    TestCase {
        name: "set_ktime_seconds",
        func: test_set_ktime_seconds,
    },
];
