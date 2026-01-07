use crate::{
    platform::{platform_get_device_info, DeviceType}, tests::TestCase
};

use super::{SerialDevice, SerialDeviceDriver, SerialManager, ns16550::Ns16550};

pub fn test_serial_subsystem_init() {
    let serial_subsystem: SerialManager = SerialManager::init();
    if serial_subsystem.get_serial_array_size() != 0 {
        panic!("Serial subsystem should be initialized empty.")
    }
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
            "Serial subsystem failed to add new serial, size should be at: 1, got: {}",
            serial_subsystem.get_serial_array_size()
        );
    }
    let serial = unsafe { &(*serial_subsystem.devices.get())[0]};
    if !serial.unwrap().default_console {
        panic!("Serial subsystem failed to set new serial as default console.");
    }
}

pub static SERIAL_SUBSYSTEM_TEST_SUITE: &[TestCase] = &[TestCase {
    name: "SerialManager::init",
    func: test_serial_subsystem_init,
}];
