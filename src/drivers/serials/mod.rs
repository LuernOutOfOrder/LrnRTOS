use core::{
    cell::UnsafeCell,
    fmt::{self, Write},
};

use ns16550::Ns16550;

use crate::{config::SERIAL_MAX_SIZE, log, logs::LogLevel};

pub mod ns16550;

#[cfg(feature = "test")]
pub mod test;

/// Generic trait to implement in each serial driver
pub trait SerialDriver: Send + Sync + Write {
    // Write char at address
    fn putchar(&self, c: u8);
    // Get char from address
    fn getchar(&self) -> u8;
}

#[cfg_attr(feature = "test", derive(Copy, Clone))]
#[derive(PartialEq)]
enum SerialDeviceDriver {
    Ns16550(Ns16550),
}

/// Generic struct for each serial device
/// id: the device id for faster access or identification
/// default_console: if it's the default console to use or not
/// driver: enum unions with all serial driver structure
#[cfg_attr(feature = "test", derive(Copy, Clone))]
#[derive(PartialEq)]
pub struct SerialDevice {
    driver: SerialDeviceDriver,
    _id: usize,
    default_console: bool,
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
    pub devices: UnsafeCell<[Option<SerialDevice>; SERIAL_MAX_SIZE]>,
}

unsafe impl Sync for SerialManager {}

impl SerialManager {
    pub const fn init() -> Self {
        SerialManager {
            devices: UnsafeCell::new([const { None }; SERIAL_MAX_SIZE]),
        }
    }

    /// Add a new serial to the device UnsafeCell array at index where there's no device
    /// By default if there's no device saved in devices, it'll set the first serial saved as
    /// default console
    pub fn add_serial(&self, new_serial: SerialDevice) {
        let mut index_none: usize = 0;
        for i in 0..SERIAL_MAX_SIZE {
            let device = unsafe { (&*self.devices.get())[i].as_ref() };
            if let Some(serial) = device {
                if serial.driver == new_serial.driver {
                    log!(
                        LogLevel::Warn,
                        "Serial-subsystem: duplicate device detected, ignoring registration request"
                    );
                    return
                }
            } else {
                index_none = i;
                break;
            }
        }
        if index_none == 0 {
            let update_serial = SerialDevice {
                _id: new_serial._id,
                default_console: true,
                driver: new_serial.driver,
            };
            unsafe { (&mut *self.devices.get())[index_none] = Some(update_serial) };
        } else {
            unsafe { (&mut *self.devices.get())[index_none] = Some(new_serial) };
        }
    }

    /// Return static reference static mutable of SerialDevice default_console
    pub fn get_default_console(&self) -> Option<&'static mut SerialDevice> {
        let devices = unsafe { &mut *self.devices.get() };
        devices
            .iter_mut()
            .find(|d| {
                if let Some(serial) = d {
                    serial.default_console
                } else {
                    false
                }
            })
            .map(|d| d.as_mut().unwrap())
    }

    pub fn get_serial_array_size(&self) -> usize {
        let mut size: usize = 0;
        for i in 0..SERIAL_MAX_SIZE {
            let present = unsafe { (&*self.devices.get())[i].is_some() };
            if present {
                size += 1;
            }
        }
        size
    }
}

pub static SERIAL_SUBSYSTEM: SerialManager = SerialManager::init();

pub fn init_serial_subsystem() {
    Ns16550::init();
    let size = SERIAL_SUBSYSTEM.get_serial_array_size();
    if size == 0 {
        panic!("Error while initializing serial sub-system, pool is empty.");
    }
}
