use core::fmt::{self, Write};

use crate::{
    devices::{devices_get_info, DeviceType},
    drivers::{serials::SERIAL_DEVICES, DriverRegion},
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

/// Static Ns16550 instance used when creating a new driver.
static mut NS16550_INSTANCE: Ns16550 = Ns16550 {
    region: DriverRegion { addr: 0, size: 0 },
};

/// Implementation of the Ns16550
impl Ns16550 {
    /// Init a new Ns16550 from the given fdt node
    pub fn init() {
        let device_info = match devices_get_info("ns16550a", DeviceType::Serial) {
            Some(d) => d,
            None => return,
        };
        let ns16550: Ns16550 = Ns16550 {
            region: device_info.header.device_addr,
        };
        unsafe { NS16550_INSTANCE = ns16550 };
        let device = UartDevice {
            _id: 0,
            default_console: false,
            // Allow static mut refs because it's only use on early boot and there's no concurrent
            // access
            #[allow(static_mut_refs)]
            driver: unsafe { &mut NS16550_INSTANCE },
        };
        SERIAL_DEVICES.add_serial(device);
    }
}
