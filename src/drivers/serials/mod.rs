use core::{cell::UnsafeCell, fmt::Write};

use ns16550::Ns16550;

use crate::config::SERIAL_MAX_SIZE;

pub mod ns16550;

/// Generic trait to implement in each uart driver
pub trait UartDriver: Send + Sync + Write {
    // Write char at address
    fn putchar(&self, c: u8);
    // Get char from address
    fn getchar(&self) -> u8;
}

/// Generic struct for each uart device
/// id: the device id for faster access or identification
/// default_console: if it's the default console to use or not
/// driver: ptr to any struct implementing the UartDriver trait
pub struct UartDevice {
    _id: usize,
    default_console: bool,
    pub driver: &'static mut dyn UartDriver,
}

/// Define and manage all serial devices.
/// Devices: use an UnsafeCell with an array of Option<UartDevice> used to store and retrieve all
/// device initialized.
pub struct SerialManager {
    // UnsafeCell array containing all serial devices
    pub devices: UnsafeCell<[Option<UartDevice>; SERIAL_MAX_SIZE]>,
}

unsafe impl Sync for SerialManager {}

impl SerialManager {
    // New without default needed for const use
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        SerialManager {
            devices: UnsafeCell::new([const { None }; SERIAL_MAX_SIZE]),
        }
    }

    /// Add a new serial to the device UnsafeCell array at index where there's no device
    /// By default if there's no device saved in devices, it'll set the first serial saved as
    /// default console
    pub fn add_serial(&self, serial: UartDevice) {
        let mut index_none: usize = 0;
        for i in 0..4 {
            let device = unsafe { (&*self.devices.get())[i].as_ref() };
            if device.is_none() {
                index_none = i;
                break;
            }
        }
        if index_none == 0 {
            let update_serial = UartDevice {
                _id: serial._id,
                default_console: true,
                driver: serial.driver,
            };
            unsafe { (&mut *self.devices.get())[index_none] = Some(update_serial) };
        } else {
            unsafe { (&mut *self.devices.get())[index_none] = Some(serial) };
        }
    }

    /// Return static reference static mutable of UartDevice default_console
    pub fn get_default_console(&self) -> Option<&'static mut UartDevice> {
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

pub static SERIAL_DEVICES: SerialManager = SerialManager::new();

pub fn init_serial_subsystem() {
    Ns16550::init();
    let size = SERIAL_DEVICES.get_serial_array_size();
    if size == 0 {
        panic!("Error while initializing serial sub-system, pool is empty.");
    }
}
