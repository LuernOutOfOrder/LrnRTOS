use crate::{
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

pub static SERIAL_SUBSYSTEM_TEST_SUITE: &[TestCase] = &[
    TestCase {
        name: "SerialManager::init",
        func: test_serial_subsystem_impl,
    },
    TestCase {
        name: "Serial sub-system add same device",
        func: test_serial_subsystem_same_device,
    },
];
