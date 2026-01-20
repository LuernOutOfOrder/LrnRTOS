use crate::{
    drivers::{
        DriverRegion,
        serials::{SerialDevice, SerialDeviceDriver, SerialDriver, ns16550a::Ns16550},
    },
    tests::{TEST_MANAGER, TestBehavior, TestCase, TestSuite, TestSuiteBehavior},
};

pub fn test_ns16550_qemu_putchar() -> u8 {
    let ns16550: Ns16550 = Ns16550 {
        region: DriverRegion {
            addr: 0x1000_0000,
            size: 0x100,
        },
    };
    let mut device = SerialDevice {
        _id: 0,
        default_console: false,
        driver: SerialDeviceDriver::Ns16550(ns16550),
    };
    // Write in buff using putchar
    match &mut device.driver {
        SerialDeviceDriver::Ns16550(ns16550) => ns16550.putchar(0x00000001),
    }
    0
}

pub fn ns16550_test_suite() {
    const NS16550_TEST_SUITE: TestSuite = TestSuite {
        tests: &[TestCase::init(
            "Ns16550 driver qemu putchar",
            test_ns16550_qemu_putchar,
            TestBehavior::Default,
        )],
        name: "Ns16550",
        tests_nb: 1,
        behavior: TestSuiteBehavior::Skipped,
    };
    #[allow(static_mut_refs)]
    unsafe {
        TEST_MANAGER.add_suite(&NS16550_TEST_SUITE)
    };
}
