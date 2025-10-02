use core::cell::UnsafeCell;

pub mod ns16550;

/// Generic trait to impl in each driver
pub trait UartDriver: Send + Sync {
    fn putchar(&self, c: u8);
    fn getchar(&self) -> u8;
}

/// Generic struct for each device
/// id: the device id for faster access or identification
/// default_console: if it's the default console to use or not
/// driver: ptr to any struct impl the UartDriver trait
#[derive(Clone, Copy)]
pub struct UartDevice {
    id: usize,
    default_console: bool,
    pub driver: &'static dyn UartDriver,
}

/// Static array containing all UART devices
pub static mut UART_DEVICES: UnsafeCell<[Option<UartDevice>; 4]> = UnsafeCell::new([None; 4]);

