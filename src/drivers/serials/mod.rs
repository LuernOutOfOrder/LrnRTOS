/*
File info: Serial devices sub-system.

Test coverage: All basic implementation and some edge cases.

Tested:
- All basic method from implementation.
- Adding the same device.
- Overflow in the sub-system pool.

Not tested:
- ...

Reasons:
- ...

Tests files:
- 'src/tests/drivers/serials/subsystem.rs'
*/

use core::{
    cell::UnsafeCell,
    fmt::{self, Write},
};

use crate::{config::SERIAL_MAX_SIZE, log, logs::LogLevel};

pub mod ns16550a;

/// Generic trait to implement in each serial driver
pub trait SerialDriver: Send + Sync + Write {
    // Write char at address
    fn putchar(&self, c: u8);
    // Get char from address
    fn getchar(&self) -> u8;
}

#[cfg_attr(feature = "test", derive(Copy, Clone))]
#[derive(PartialEq)]
pub enum SerialDeviceDriver {
    Ns16550(ns16550a::Ns16550),
}

/// Generic struct for each serial device
/// id: the device id for faster access or identification
/// default_console: if it's the default console to use or not
/// driver: enum unions with all serial driver structure
#[cfg_attr(feature = "test", derive(Copy, Clone))]
#[derive(PartialEq)]
pub struct SerialDevice {
    pub driver: SerialDeviceDriver,
    pub _id: usize,
    pub default_console: bool,
}

impl SerialDevice {
    pub fn write_fmt(&mut self, s: core::fmt::Arguments) -> fmt::Result {
        match &mut self.driver {
            SerialDeviceDriver::Ns16550(ns16550) => ns16550.write_fmt(s),
        }
    }
}

/// Define and manage all serial devices.
/// Devices: use an UnsafeCell with an array of Option<UartDevice> used to store and retrieve all
/// device initialized.
pub struct SerialManager {
    pub devices: [UnsafeCell<Option<SerialDevice>>; SERIAL_MAX_SIZE],
}

unsafe impl Sync for SerialManager {}

impl SerialManager {
    pub const fn init() -> Self {
        SerialManager {
            // devices: UnsafeCell::new([const { None }; SERIAL_MAX_SIZE]),
            devices: [const { UnsafeCell::new(const { None }) }; SERIAL_MAX_SIZE],
        }
    }

    /// Add a new serial to the device UnsafeCell array at index where there's no device
    /// By default if there's no device saved in devices, it'll set the first serial saved as
    /// default console
    pub fn add_serial(&self, new_serial: SerialDevice) {
        let mut index_none: Option<usize> = None;
        for i in 0..SERIAL_MAX_SIZE {
            let device = unsafe { &*self.devices[i].get() };
            if let Some(serial) = device {
                // Avoid duplication and log warning
                if serial.driver == new_serial.driver {
                    log!(
                        LogLevel::Warn,
                        "Serial sub-system: duplicate device detected, ignoring registration request"
                    );
                    return;
                }
            } else if device.is_none() {
                index_none = Some(i);
                break;
            }
        }
        if index_none.is_none() {
            log!(
                LogLevel::Warn,
                "Serial sub-system: subsystem is full, ignoring registration request"
            );
            return;
        }
        // Set default console
        if index_none.unwrap() == 0 {
            let update_serial = SerialDevice {
                _id: new_serial._id,
                default_console: true,
                driver: new_serial.driver,
            };
            unsafe { *self.devices[0].get() = Some(update_serial) }
        } else {
            // Just save the new device
            unsafe { *self.devices[index_none.unwrap()].get() = Some(new_serial) };
        }
    }

    /// Return &mut default_console from subsystem, 
    ///
    /// # Safety
    ///
    /// - currently the kernel is single-threaded
    /// - interrupt system can't access the sub-system concurrently
    ///   unsafe function while there's no mutex built
    #[allow(clippy::mut_from_ref)]
    pub unsafe fn get_default_console(&self) -> &mut SerialDevice {
        let default_console = unsafe { (*self.devices[0].get()).as_mut() };
        if let Some(serial) = default_console {
            serial
        } else {
            panic!(
                "Serial sub-system: invariant violated, default console should always be available after the sub-system initialized"
            );
        }
    }

    pub fn get_serial_array_size(&self) -> usize {
        let mut size: usize = 0;
        for i in 0..SERIAL_MAX_SIZE {
            let present = unsafe { &*self.devices[i].get() };
            if present.is_some() {
                size += 1;
            }
        }
        size
    }
}

pub static SERIAL_SUBSYSTEM: SerialManager = SerialManager::init();

pub fn init_serial_subsystem() {
    ns16550a::Ns16550::init();
    let size = SERIAL_SUBSYSTEM.get_serial_array_size();
    if size == 0 {
        panic!("Error while initializing serial sub-system, pool is empty.");
    }
}
