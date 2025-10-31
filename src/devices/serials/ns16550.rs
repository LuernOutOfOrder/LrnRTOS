use core::fmt::{self, Write};

use crate::{
    devices::serials::UART_DEVICES,
    dtb::{FdtNode, helpers::get_node_prop_in_hierarchy},
    kprint,
};

use super::{UartDevice, UartDriver};

pub struct Ns16550 {
    pub addr: usize,
}

impl UartDriver for Ns16550 {
    fn putchar(&self, c: u8) {
        unsafe { core::ptr::write_volatile(self.addr as *mut u8, c) }
    }
    fn getchar(&self) -> u8 {
        todo!()
    }
}

impl Write for Ns16550 {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            self.putchar(b);
        }
        Ok(())
    }
}

impl Ns16550 {
    pub fn init(node: &FdtNode) {
        let address_cells = get_node_prop_in_hierarchy(node, "#address-cells");
        let size_cells = get_node_prop_in_hierarchy(node, "#size-cells");
        static mut NS16550: Ns16550 = Ns16550 { addr: 0x10000000 };
        let devices = unsafe { &mut *UART_DEVICES.get() };
        // Basic loop and no iter.position ??
        (0..devices.len()).for_each(|i| {
            if devices[i].is_none() {
                devices[i] = Some(UartDevice {
                    id: 0,
                    default_console: false,
                    driver: unsafe { &mut NS16550 },
                })
            }
        });
    }
}
