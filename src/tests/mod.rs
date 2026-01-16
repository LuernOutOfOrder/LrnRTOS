use core::ptr;

pub mod arch;
pub mod drivers;
pub mod ktime;
pub mod mem;
pub mod platform;

use arch::traps::{
    handler::trap_handler_test_suite, interrupt::interrupt_enabling_test_suite,
    trap_frame::trap_frame_test_suite,
};
use drivers::{
    cpu_intc::subsystem::cpu_intc_subsystem_test_suite,
    serials::{ns16550a::ns16550_test_suite, subsystem::serial_subsystem_test_suite},
    timer::subsystem::timer_subsystem_test_suite,
};
use ktime::ktime_test_suite;
use mem::memory_test_suite;
use platform::{platform_test_suite, test_platform_init};

use crate::{kprint, kprint_fmt};

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
macro_rules! test_info {
    ($($arg:tt)*) => {
        $crate::tests::test_info_kprint(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! test_failed {
    ($($arg:tt)*) => {
        $crate::tests::test_failed(format_args!($($arg)*))
    };
}

pub fn test_info_kprint(s: core::fmt::Arguments) {
    kprint_fmt!("\x1b[33;1m[TEST INFO]\x1b[0m {}\n", s);
}

pub fn test_failed(s: core::fmt::Arguments) {
    kprint_fmt!("\x1b[31;1m[TEST FAILED]\x1b[0m {}\n", s);
}

#[panic_handler]
pub fn test_panic(s: &core::panic::PanicInfo) -> ! {
    kprint_fmt!("\x1b[31;1m[KERNEL INTEGRITY FAILURE]\x1b[0m {:?}", s);
    loop {}
}

pub struct TestManager<'a> {
    // Represent the next empty index to push new test suite, also used to know how many test suite
    // in test_pool by suite_nb - 1.
    pub test_pool: [TestSuite<'a>; 20],
    pub suite_nb: Option<usize>,
    pub suite_passed: usize,
    pub suite_failed: usize,
}

impl<'a> TestManager<'a> {
    pub const fn init() -> Self {
        TestManager {
            test_pool: [TestSuite::init_default(); 20],
            suite_nb: None,
            suite_passed: 0,
            suite_failed: 0,
        }
    }

    pub fn add_suite(&'a mut self, new_test_suite: &'a TestSuite) {
        if self.suite_nb.is_none() {
            self.test_pool[0] = *new_test_suite;
            self.suite_nb = Some(1)
        } else {
            self.test_pool[self.suite_nb.unwrap()] = *new_test_suite;
            if let Some(nb) = self.suite_nb.as_mut() {
                *nb += 1;
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct TestSuite<'a> {
    pub tests: &'a [TestCase<'a>],
    pub name: &'a str,
    pub tests_nb: u32,
}

impl<'a> TestSuite<'a> {
    pub const fn init_default() -> Self {
        TestSuite {
            tests: &[],
            name: "",
            tests_nb: 0,
        }
    }

    pub fn init(tests: &'a [TestCase], name: &'a str, tests_nb: u32) -> Self {
        TestSuite {
            tests,
            name,
            tests_nb,
        }
    }
}

#[derive(Copy, Clone)]
pub enum TestBehavior {
    Default,
    ShouldFailed,
}

#[derive(Copy, Clone)]
pub struct TestCase<'a> {
    pub name: &'a str,
    pub func: fn() -> u8,
    pub behavior: TestBehavior,
}

impl<'a> TestCase<'a> {
    const fn init(name: &'a str, func: fn() -> u8, behavior: TestBehavior) -> Self {
        TestCase {
            name,
            func,
            behavior,
        }
    }
}

pub static mut TEST_MANAGER: TestManager = TestManager::init();

// Call all test suite function to auto register all suites in test manager.
fn test_suites() {
    platform_test_suite();
    serial_subsystem_test_suite();
    timer_subsystem_test_suite();
    cpu_intc_subsystem_test_suite();
    ktime_test_suite();
    ns16550_test_suite();
    trap_frame_test_suite();
    interrupt_enabling_test_suite();
    trap_handler_test_suite();
    memory_test_suite();
}

#[unsafe(no_mangle)]
pub fn test_runner(core: usize, dtb_addr: usize) -> ! {
    // Basic test before running all test suites
    kprint!("Starting kernel in test mode.\n");
    if core != 0 {
        panic!("Booting on wrong CPU core");
    }
    test_kprint!("Successfully start kernel booting on CPU Core: 0.");
    test_info!("Running test: platform_init");
    test_platform_init(dtb_addr);
    test_kprint!("platform_init");
    // All test suites
    test_suites();

    let test_suites_nb = unsafe { TEST_MANAGER.suite_nb.unwrap() };
    kprint_fmt!("\nNumber of test suites to run: {}\n\n", test_suites_nb);

    // Tests
    let mut tests_run: usize = 0;
    let mut tests_sucess: usize = 0;
    let mut tests_failed: usize = 0;
    // Tests suites
    let mut test_suites_failed: usize = 0;
    // Iterate over all test suite and run all test inside
    for test_suite in unsafe { TEST_MANAGER.test_pool } {
        if test_suite.tests_nb == 0 {
            break;
        }
        kprint_fmt!(
            "\nRunning {} tests from test suite: {}\n",
            test_suite.tests_nb,
            test_suite.name
        );
        let test_to_pass = test_suite.tests_nb;
        let mut test_passed: usize = 0;
        let mut test_failed: usize = 0;
        for test in test_suite.tests {
            test_info!("Running test: {}", test.name);
            let pass = (test.func)();
            if pass == 1 {
                test_failed += 1;
                test_suites_failed += 1;
                break;
            }
            test_kprint!("{}", test.name);
            test_passed += 1;
        }
        tests_run += test_to_pass as usize;
        tests_sucess += test_passed;
        tests_failed += test_failed;
        kprint!("Test suite passed successfully\n");
    }
    kprint_fmt!(
        "\nTests ran: {}\ttests passed: {}/{}\ttests failed: {}\n",
        tests_run,
        tests_sucess,
        tests_run,
        tests_failed
    );
    let test_suites_passed = test_suites_nb - test_suites_failed;
    kprint_fmt!(
        "Test suites ran: {}\ttest suites passed: {}/{}\ttest suites failed: {}\n",
        test_suites_nb,
        test_suites_passed,
        test_suites_nb,
        test_suites_failed
    );
    // Exit Qemu at the end of the tests
    unsafe { ptr::write_volatile(0x100000 as *mut u32, 0x5555) };
    #[allow(clippy::empty_loop)]
    loop {}
}
