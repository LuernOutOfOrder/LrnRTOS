use crate::{
    drivers::DriverRegion,
    kprint_fmt,
    platform::{DeviceType, platform_get_device_info},
    tests::TestCase,
};

use super::{SerialDevice, SerialDeviceDriver, SerialManager, ns16550::Ns16550};

pub fn test_serial_subsystem_impl() {
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
    let serial = unsafe { &(*serial_subsystem.devices.get())[0] };
    if !serial.unwrap().default_console {
        panic!("Serial sub-system failed to set new serial as default console.");
    }

    let default_console = serial_subsystem.get_default_console().unwrap();
    if *default_console != serial.unwrap() {
        panic!(
            "Error getting the default console, default console get is different than the one saved before."
        );
    }
}

/// Test how the sub-system react when adding multiple time the same device.
pub fn test_serial_subsystem_same_device() {
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
    Ns16550::init();
    serial_subsystem.add_serial(device);
}

pub fn test_serial_subsystem_overflow() {
    let serial_subsystem: SerialManager = SerialManager::init();
    if serial_subsystem.get_serial_array_size() != 0 {
        panic!("Serial sub-system should be initialized empty.")
    }
    // Build multiple serial to test how the subsystem handle overflow
    // First Serial
    let ns16550: Ns16550 = Ns16550 {
        region: DriverRegion {
            addr: 0x10000000,
            size: 0x100,
        },
    };
    let device = SerialDevice {
        _id: 0,
        default_console: false,
        driver: SerialDeviceDriver::Ns16550(ns16550),
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
    serial_subsystem.add_serial(device);
    serial_subsystem.add_serial(device1);
    serial_subsystem.add_serial(device2);
    serial_subsystem.add_serial(device3);
    // This one should trigger a warning and not be registered to the sub-system
    serial_subsystem.add_serial(device4);

    // Check default console
    // Unwrap because we know that there's a device
    // Get default console MMIO reg
    let default_console = serial_subsystem.get_default_console().unwrap();
    let default_console_region = {
        match default_console.driver {
            SerialDeviceDriver::Ns16550(ns16550) => ns16550.region,
        }
    };
    // Get first device registered MMIO reg
    let device_region = {
        match device.driver {
            SerialDeviceDriver::Ns16550(ns16550) => ns16550.region,
        }
    };
    // Compare default console with first device registered
    if default_console_region != device_region {
        panic!("Wrong default console. The default console should be the first device registered.")
    }
    // Check last device registered
    let last_device = unsafe { (&*serial_subsystem.devices.get())[3] };
    let last_device_region = {
        // Unwrap because we know it's Some
        match last_device.unwrap().driver {
            SerialDeviceDriver::Ns16550(ns16550) => ns16550.region,
        }
    };
    let device3_region = {
        match device3.driver {
            SerialDeviceDriver::Ns16550(ns16550) => ns16550.region,
        }
    };
    if last_device_region != device3_region {
        panic!("Wrong last device registered. The last device should not be replaced when possible overflow happened.")
    }
}

pub static SERIAL_SUBSYSTEM_TEST_SUITE: &[TestCase] = &[
    TestCase {
        name: "SerialManager::init",
        func: test_serial_subsystem_impl,
    },
    TestCase {
        name: "Serial sub-system add same device",
        func: test_serial_subsystem_same_device,
    },
    TestCase {
        name: "Serial sub-system handling overflow",
        func: test_serial_subsystem_overflow,
    },
];
