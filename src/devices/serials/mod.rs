use core::{cell::UnsafeCell, fmt::Write, mem::MaybeUninit};

pub mod ns16550;

/// Generic trait to impl in each driver
pub trait UartDriver: Send + Sync + Write {
    fn putchar(&self, c: u8);
    fn getchar(&self) -> u8;
}

/// Generic struct for each device
/// id: the device id for faster access or identification
/// default_console: if it's the default console to use or not
/// driver: ptr to any struct impl the UartDriver trait
pub struct UartDevice {
    id: usize,
    default_console: bool,
    pub driver: &'static mut dyn UartDriver,
}

/// Global ptr for default kernel console
pub static mut KCONSOLE: UnsafeCell<Option<&'static mut dyn Write>> = UnsafeCell::new(None);

pub fn set_kconsole(writer: &'static mut dyn Write) {
    unsafe {
        *KCONSOLE.get() = Some(writer);
    }
}

/// Global static array containing all UART devices
pub static mut UART_DEVICES: UnsafeCell<[Option<UartDevice>; 4]> =
    UnsafeCell::new([const { None }; 4]);
//
// static mut UART_DEVICES: [MaybeUninit<UartDevice>; 4] =
//     [MaybeUninit::uninit(); 4];
