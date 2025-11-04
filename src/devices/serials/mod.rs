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
    _default_console: bool,
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

/// Global static array containing all UART devices
pub static mut UART_DEVICES: UnsafeCell<[Option<UartDevice>; 4]> =
    UnsafeCell::new([const { None }; 4]);
