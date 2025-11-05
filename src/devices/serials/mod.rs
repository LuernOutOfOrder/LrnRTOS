use core::{cell::UnsafeCell, fmt::Write};

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

/// Global ptr for default kernel console
/// Principally used for debugging when no uart is initialized
pub static mut KCONSOLE: UnsafeCell<Option<&'static mut dyn Write>> = UnsafeCell::new(None);

/// Set KCONSOLE from any structure implementing Write trait
pub fn set_kconsole(writer: &'static mut dyn Write) {
    unsafe {
        *KCONSOLE.get() = Some(writer);
    }
}

/// Define and manage all serial devices.
/// devices: use an UnsafeCell with an array of Option<UartDevice> used to store and retrieve all
/// device initialized.
pub struct SerialManager {
    // UnsafeCell array containing all serial devices
    pub devices: UnsafeCell<[Option<UartDevice>; 4]>,
}

unsafe impl Sync for SerialManager {}

impl SerialManager {
    pub const fn new() -> Self {
        SerialManager {
            devices: UnsafeCell::new([const { None }; 4]),
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
}

pub static SERIAL_DEVICES: SerialManager = SerialManager::new();
