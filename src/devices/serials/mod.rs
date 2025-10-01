mod ns16550;

/// Generic trait to impl in each driver
pub trait UartDriver {
    fn init(&self);
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
    driver: &'static dyn UartDriver,
}

/// 
pub struct UartDevices {
    devices: [UartDevice; 10],
}

