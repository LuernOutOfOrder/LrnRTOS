use crate::devices::serials::UART_DEVICES;

use super::{UartDevice, UartDriver};

pub struct Ns16550 {
    addr: usize,
}

impl UartDriver for Ns16550 {
    fn init(&self) {
        static NS16550: Ns16550 = Ns16550 { addr: 0x1000000 };
        let device: UartDevice = UartDevice {
            id: 0,
            default_console: false,
            driver: &NS16550,
        };
        unsafe {
            if let Some(i) = UART_DEVICES.iter().position(|x| x.is_none()) {
                UART_DEVICES[i] = core::prelude::v1::Some(device);
            }
        }
    }
    fn putchar(&self, char: u8) {
        unsafe { core::ptr::write_volatile(self.addr as *mut u8, char) }
    }
    fn getchar(&self) -> u8 {
        todo!()
    }
}
