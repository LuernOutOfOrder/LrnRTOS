use core::fmt::{self, Write};

use crate::{
    drivers::DriverRegion,
    platform::{DeviceType, platform_get_device_info},
};

use super::{UartDevice, UartDriver};

/// Structure for Ns16550 driver
/// region: DriverRegion struct to define address memory region to use with the driver and the address size
pub struct Ns16550 {
    pub region: DriverRegion,
}

/// Implementing the UartDriver trait for Ns16550 driver
impl UartDriver for Ns16550 {
    fn putchar(&self, c: u8) {
        unsafe { core::ptr::write_volatile(self.region.addr as *mut u8, c) }
    }
    fn getchar(&self) -> u8 {
        todo!()
    }
}

/// Implementing Write trait for Ns16550 to be able to format with core::fmt in print
/// Use the UartDriver function implemented in Ns16550
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
    /// Init a new Ns16550 from the given fdt node
    pub fn init() -> Option<UartDevice> {
        let device_info = platform_get_device_info("ns16550a", DeviceType::Serial)?;
        let ns16550: Ns16550 = Ns16550 {
            region: device_info.header.device_addr,
        };
        let device = UartDevice {
            _id: 0,
            default_console: false,
            driver: super::UartDeviceDriverType::Ns16550(ns16550),
        };
        Some(device)
    }
}
