use crate::devices::serials::UART_DEVICES;

use super::{UartDevice, UartDriver};

pub struct Ns16550 {
    pub addr: usize,
}

impl UartDriver for Ns16550 {
    fn putchar(&self, char: u8) {
        unsafe { core::ptr::write_volatile(self.addr as *mut u8, char) }
    }
    fn getchar(&self) -> u8 {
        todo!()
    }
}

impl Ns16550 {
    pub fn init() {
        static NS16550: Ns16550 = Ns16550 { addr: 0x10000000 };
        let device: UartDevice = UartDevice {
            id: 0,
            default_console: false,
            driver: &NS16550,
        };
        let devices = unsafe { &mut *UART_DEVICES.get() };
        // Basic loop and no iter.position ??
        (0..devices.len()).for_each(|i| {
            if devices[i].is_none() {
                devices[i] = Some(device)
            }
        });
    }
}
