use crate::{
    drivers::{
        DriverRegion,
        serials::{SerialDevice, SerialDeviceDriver, SerialDriver, ns16550::Ns16550},
    },
    tests::{TEST_MANAGER, TestCase},
};

pub fn test_ns16550_qemu_putchar() {
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
}

pub fn ns16550_test_suite() {
    let ns16550_test_suite: &[TestCase] = &[TestCase {
        name: "Ns16550 driver qemu putchar",
        func: test_ns16550_qemu_putchar,
    }];
    #[allow(static_mut_refs)]
    unsafe {
        TEST_MANAGER.add_suite(ns16550_test_suite)
    };
}
