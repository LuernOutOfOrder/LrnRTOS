use core::ptr;

pub mod drivers;

use drivers::{cpu_intc::subsystem::CPU_INTC_SUBSYSTEM_TEST_SUITE, serials::subsystem::SERIAL_SUBSYSTEM_TEST_SUITE, timer::subsystem::TIMER_SUBSYSTEM_TEST_SUITE};

use crate::{
    kprint_fmt, platform::test::PLATFORM_TEST_SUITE
};

#[macro_export]
macro_rules! test_kprint {
    ($($arg:tt)*) => {
        $crate::tests::test_kprint(format_args!($($arg)*))
    };
}

pub fn test_kprint(s: core::fmt::Arguments) {
    kprint_fmt!("\x1b[32;1m[TEST PASSED]\x1b[0m {}\n", s);
}

#[macro_export]
macro_rules! test_info_kprint {
    ($($arg:tt)*) => {
        $crate::tests::test_info_kprint(format_args!($($arg)*))
    };
}

macro_rules! run_test {
    ($test_name: expr, $fn: expr) => {
        test_info_kprint!("Running test: {}", $test_name);
        ($fn)();
        test_kprint!("{}", $test_name);
    };
}

pub fn test_info_kprint(s: core::fmt::Arguments) {
    kprint_fmt!("\x1b[33;1m[TEST INFO]\x1b[0m {}\n", s);
}

#[panic_handler]
pub fn test_panic(s: &core::panic::PanicInfo) -> ! {
    kprint_fmt!("\x1b[31;1m[TEST FAILED]\x1b[0m {:?}", s);
    loop {}
}

pub struct TestCase<'a> {
    pub name: &'a str,
    pub func: fn(),
}

// Static containing all test suite
static TEST_SUITE: &[&[TestCase]] = &[
    PLATFORM_TEST_SUITE,
    SERIAL_SUBSYSTEM_TEST_SUITE,
    TIMER_SUBSYSTEM_TEST_SUITE,
    CPU_INTC_SUBSYSTEM_TEST_SUITE,
];

#[unsafe(no_mangle)]
pub fn test_runner(core: usize, dtb_addr: usize) -> ! {
    use crate::platform::test::test_platform_init;

    test_info_kprint!("Starting kernel in test mode.");
    if core != 0 {
        panic!("Booting on wrong CPU core");
    }
    test_kprint!("Successfully start kernel booting on CPU Core: 0.");
    test_info_kprint!("Running test: platform_init");
    test_platform_init(dtb_addr);
    test_kprint!("platform_init");

    // Iterate over all test suite and run all test inside
    for test_suite in TEST_SUITE {
        for test in *test_suite {
            run_test!(test.name, test.func);
        }
    }
    // Exit Qemu at the end of the tests
    unsafe { ptr::write_volatile(0x100000 as *mut u32, 0x5555) };
    #[allow(clippy::empty_loop)]
    loop {}
}
