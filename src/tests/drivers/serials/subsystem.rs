use crate::{
    drivers::{
        serials::{
            init_serial_subsystem, ns16550a::Ns16550, SerialDevice, SerialDeviceDriver, SerialManager
        }, DriverRegion
    },
    platform::{platform_get_device_info, DeviceType},
    tests::{TestBehavior, TestCase, TestSuite, TestSuiteBehavior, TEST_MANAGER},
};

pub fn test_serial_subsystem_impl() -> u8 {
    // Check init serial subsystem
    let serial_subsystem: SerialManager = SerialManager::init();
    if serial_subsystem.get_serial_array_size() != 0 {
        panic!("Serial sub-system should be initialized empty.")
    }
    // Add serial to subsystem
    // Just unwrap, being in test env we know that it will return Some.
    let device_info = platform_get_device_info("ns16550a", DeviceType::Serial).unwrap();
    let ns16550: Ns16550 = Ns16550 {
        region: device_info.header.device_addr,
    };
    let device = SerialDevice {
        _id: 0,
        default_console: false,
        driver: SerialDeviceDriver::Ns16550(ns16550),
    };
    serial_subsystem.add_serial(device);
    if serial_subsystem.get_serial_array_size() != 1 {
        panic!(
            "Serial sub-system failed to add new serial, size should be at: 1, got: {}",
            serial_subsystem.get_serial_array_size()
        );
    }
    // Check default console
    let serial = unsafe { &*serial_subsystem.devices[0].get() };
    if !serial.as_ref().unwrap().default_console {
        panic!("Serial sub-system failed to set new serial as default console.");
    }

    let default_console = unsafe { serial_subsystem.get_default_console() };
    if default_console != serial.as_ref().unwrap() {
        panic!(
            "Error getting the default console, default console get is different than the one saved before."
        );
    }

    // Initialize serial subsystem, don't know where to put it instead of here
    init_serial_subsystem();
    0
}

/// Test how the sub-system react when adding multiple time the same device.
pub fn test_serial_subsystem_same_device() -> u8 {
    let serial_subsystem: SerialManager = SerialManager::init();
    if serial_subsystem.get_serial_array_size() != 0 {
        panic!("Serial sub-system should be initialized empty.")
    }
    // Add serial to subsystem
    // Just unwrap, being in test env we know that it will return Some.
    let device_info = platform_get_device_info("ns16550a", DeviceType::Serial).unwrap();
    let ns16550: Ns16550 = Ns16550 {
        region: device_info.header.device_addr,
    };
    let device = SerialDevice {
        _id: 0,
        default_console: false,
        driver: SerialDeviceDriver::Ns16550(ns16550),
    };
    // Add second serial to subsystem
    let device_info = platform_get_device_info("ns16550a", DeviceType::Serial).unwrap();
    let ns16551: Ns16550 = Ns16550 {
        region: device_info.header.device_addr,
    };
    let device1 = SerialDevice {
        _id: 0,
        default_console: false,
        driver: SerialDeviceDriver::Ns16550(ns16551),
    };
    // All same devices
    serial_subsystem.add_serial(device);
    // This one should trigger a warning
    serial_subsystem.add_serial(device1);
    if serial_subsystem.get_serial_array_size() != 1 {
        panic!("Serial sub-system should contain only 1 device.");
    }
    0
}

pub fn test_serial_subsystem_overflow() -> u8 {
    let serial_subsystem: SerialManager = SerialManager::init();
    if serial_subsystem.get_serial_array_size() != 0 {
        panic!("Serial sub-system should be initialized empty.")
    }
    // Build multiple serial to test how the subsystem handle overflow
    // First Serial
    const NS16550: Ns16550 = Ns16550 {
        region: DriverRegion {
            addr: 0x10000000,
            size: 0x100,
        },
    };
    const DEVICE: SerialDevice = SerialDevice {
        _id: 0,
        default_console: false,
        driver: SerialDeviceDriver::Ns16550(NS16550),
    };
    // Second Serial
    let ns16551: Ns16550 = Ns16550 {
        region: DriverRegion {
            addr: 0x10000001,
            size: 0x200,
        },
    };
    let device1 = SerialDevice {
        _id: 0,
        default_console: false,
        driver: SerialDeviceDriver::Ns16550(ns16551),
    };
    // Third Serial
    let ns16552: Ns16550 = Ns16550 {
        region: DriverRegion {
            addr: 0x10000002,
            size: 0x300,
        },
    };
    let device2 = SerialDevice {
        _id: 0,
        default_console: false,
        driver: SerialDeviceDriver::Ns16550(ns16552),
    };
    // Fourth serial
    let ns16553: Ns16550 = Ns16550 {
        region: DriverRegion {
            addr: 0x10000003,
            size: 0x400,
        },
    };
    let device3 = SerialDevice {
        _id: 0,
        default_console: false,
        driver: SerialDeviceDriver::Ns16550(ns16553),
    };
    // Last serial, cause the overflow
    let ns16554: Ns16550 = Ns16550 {
        region: DriverRegion {
            addr: 0x10000004,
            size: 0x500,
        },
    };
    let device4 = SerialDevice {
        _id: 0,
        default_console: false,
        driver: SerialDeviceDriver::Ns16550(ns16554),
    };
    // Register all devices
    serial_subsystem.add_serial(DEVICE);
    serial_subsystem.add_serial(device1);
    serial_subsystem.add_serial(device2);
    serial_subsystem.add_serial(device3);
    // Save the state of the serial subsystem
    // Manually copy the inner values since UnsafeCell doesn't implement Clone
    let serial_subsystem_snapshot = unsafe {
        [
            &*serial_subsystem.devices[0].get(),
            &*serial_subsystem.devices[1].get(),
            &*serial_subsystem.devices[2].get(),
            &*serial_subsystem.devices[3].get(),
        ]
    };
    // This one should trigger a warning and not be registered to the sub-system
    serial_subsystem.add_serial(device4);
    // Check if the subsystem has changed after the overflow aborted
    let current_devices = unsafe {
        [
            &*serial_subsystem.devices[0].get(),
            &*serial_subsystem.devices[1].get(),
            &*serial_subsystem.devices[2].get(),
            &*serial_subsystem.devices[3].get(),
        ]
    };
    if serial_subsystem_snapshot != current_devices {
        panic!(
            "Serial sub-system state has changed after handling the overflow. This should not happened"
        );
    }

    // Check default console
    // Unwrap because we know that there's a device
    // Get default console MMIO reg
    let default_console = unsafe { serial_subsystem.get_default_console() };
    let default_console_region = {
        match &default_console.driver {
            SerialDeviceDriver::Ns16550(ns16550) => ns16550.region,
        }
    };
    // Get first device registered MMIO reg
    let device_region = {
        match DEVICE.driver {
            SerialDeviceDriver::Ns16550(ns16550) => ns16550.region,
        }
    };
    // Compare default console with first device registered
    if default_console_region != device_region {
        panic!("Wrong default console. The default console should be the first device registered.")
    }
    0
}

pub fn serial_subsystem_test_suite() {
    const SERIAL_SUBSYSTEM_TEST_SUITE: TestSuite = TestSuite {
        tests: &[
            TestCase::init(
                "Serial sub-system basic implementation",
                test_serial_subsystem_impl,
                TestBehavior::Default,
            ),
            TestCase::init(
                "Serial sub-system add same device",
                test_serial_subsystem_same_device,
                TestBehavior::Default,
            ),
            TestCase::init(
                "Serial sub-system handling overflow",
                test_serial_subsystem_overflow,
                TestBehavior::Default,
            ),
        ],
        name: "Serial sub-system",
        tests_nb: 3,
        behavior: TestSuiteBehavior::Default
    };
    #[allow(static_mut_refs)]
    unsafe {
        TEST_MANAGER.add_suite(&SERIAL_SUBSYSTEM_TEST_SUITE)
    };
}
