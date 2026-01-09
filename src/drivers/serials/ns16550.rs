use core::fmt::{self, Write};

use crate::{
    drivers::DriverRegion,
    platform::{DeviceType, platform_get_device_info},
};

use super::{SERIAL_SUBSYSTEM, SerialDevice, SerialDeviceDriver, SerialDriver};

/// Structure for Ns16550 driver
/// region: DriverRegion struct to define address memory region to use with the driver and the address size
#[cfg_attr(feature = "test", derive(Copy, Clone))]
#[derive(PartialEq)]
pub struct Ns16550 {
    pub region: DriverRegion,
}

/// Implementing the SerialDriver trait for Ns16550 driver
impl SerialDriver for Ns16550 {
    fn putchar(&self, c: u8) {
        unsafe { core::ptr::write_volatile(self.region.addr as *mut u8, c) }
    }
    fn getchar(&self) -> u8 {
        todo!()
    }
}

/// Implementing Write trait for Ns16550 to be able to format with core::fmt in print
/// Use the SerialDriver function implemented in Ns16550
impl Write for Ns16550 {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            self.putchar(b);
        }
        Ok(())
    }
}

/// Implementation of the Ns16550
impl Ns16550 {
    /// Init a new Ns16550 from the platform layer
    pub fn init() {
        let device_info = match platform_get_device_info("ns16550a", DeviceType::Serial) {
            Some(d) => d,
            None => return,
        };
        // Check MMIO reg
        if device_info.header.device_addr.addr == 0 {
            panic!(
                "Encounter a wrong MMIO reg when initializing device. Check the device definition or hardware."
            );
        }
        if device_info.header.device_addr.size == 0 {
            panic!(
                "Encounter a wrong MMIO reg size when initializing device. Check the device definition or hardware."
            );
        }
        let ns16550: Ns16550 = Ns16550 {
            region: device_info.header.device_addr,
        };
        let device = SerialDevice {
            _id: 0,
            default_console: false,
            driver: SerialDeviceDriver::Ns16550(ns16550),
        };
        SERIAL_SUBSYSTEM.add_serial(device);
    }
}
