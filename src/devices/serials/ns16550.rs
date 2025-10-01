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
        unsafe {
            // Basic loop and no iter.position ??
            if let Some(i) = UART_DEVICES.iter().position(|x| x.is_none()) {
                UART_DEVICES[0] = Some(device);
            }
        }
    }
}
