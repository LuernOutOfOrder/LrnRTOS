use core::ptr;

use crate::{
    drivers::{
        cpu_intc::test::CPU_INTC_SUBSYSTEM_TEST_SUITE, serials::test::SERIAL_SUBSYSTEM_TEST_SUITE,
        timer::test::TIMER_SUBSYSTEM_TEST_SUITE,
    },
    kprint_fmt,
    platform::test::PLATFORM_TEST_SUITE,
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

    // Platform test suite
    let platform_tests = PLATFORM_TEST_SUITE;
    for test in platform_tests {
        run_test!(test.name, test.func);
    }
    // Serial subsystem test suite
    let serial_subsystem_tests = SERIAL_SUBSYSTEM_TEST_SUITE;
    for test in serial_subsystem_tests {
        run_test!(test.name, test.func);
    }
    // Timer subsystem test suite
    let timer_subsystem_tests = TIMER_SUBSYSTEM_TEST_SUITE;
    for test in timer_subsystem_tests {
        run_test!(test.name, test.func);
    }
    // CPU interrupt-controller subsystem test suite
    let cpu_intc_tests = CPU_INTC_SUBSYSTEM_TEST_SUITE;
    for test in cpu_intc_tests {
        run_test!(test.name, test.func);
    }
    // Exit Qemu at the end of the tests
    unsafe { ptr::write_volatile(0x100000 as *mut u32, 0x5555) };
    #[allow(clippy::empty_loop)]
    loop {}
}
