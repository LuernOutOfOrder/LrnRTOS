use core::{
    fmt::{self, Write},
    ptr,
};

use arrayvec::ArrayVec;

use crate::{
    devices::{DriverRegion, serials::UART_DEVICES},
    dtb::{
        FdtNode,
        helpers::{get_node_prop, get_node_prop_in_hierarchy},
    },
    kprint,
};

use super::{UartDevice, UartDriver};

pub struct Ns16550 {
    pub region: DriverRegion,
}

impl UartDriver for Ns16550 {
    fn putchar(&self, c: u8) {
        unsafe { core::ptr::write_volatile(self.region.addr as *mut u8, c) }
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
        // Get address and size cells
        let address_cells = get_node_prop_in_hierarchy(node, "#address-cells").unwrap();
        let size_cells = get_node_prop_in_hierarchy(node, "#size-cells").unwrap();
        // Ptr read address and size cells value from off and cast it to u32 target's endianness
        let address_cells_val: u32 =
            u32::from_be(unsafe { ptr::read(address_cells.off_value as *const u32) });
        let size_cells_val: u32 =
            u32::from_be(unsafe { ptr::read(size_cells.off_value as *const u32) });
        // Get device memory region
        let reg = get_node_prop(node, "reg").unwrap();
        let mut reg_buff: ArrayVec<u32, 120> = ArrayVec::new();
        let mut reg_cursor = reg.off_value;
        // Divide reg.value_len by 4 because we read u32 and not u8
        for _ in 0..reg.value_len / 4 {
            let value = u32::from_be(unsafe { ptr::read(reg_cursor as *const u32) });
            reg_buff.push(value);
            reg_cursor += 4;
        }
        let reg_size = address_cells_val + size_cells_val;
        let mut device_addr: DriverRegion = DriverRegion { addr: 0, size: 0 };
        for addr in reg_buff.chunks(reg_size as usize) {
            // Build addr from chunk
            let mut device_addr_build: u64 = 0;
            for i in 0..address_cells_val {
                device_addr_build = (device_addr_build << 32) | (addr[i as usize] as u64);
            }
            // Build size from chunk
            let mut device_size: u64 = 0;
            for i in 0..size_cells_val {
                device_size =
                    (device_size << 32) | (addr[address_cells_val as usize + i as usize] as u64);
            }
            device_addr = DriverRegion {
                addr: device_addr_build as usize,
                size: device_size as usize,
            }
        }
        kprint!("debug: {:#x}\n", device_addr.size);
        // let mut NS16550: Ns16550 = Ns16550 { region: device_addr };
        // let devices = unsafe { &mut *UART_DEVICES.get() };
        // // Basic loop and no iter.position ??
        // (0..devices.len()).for_each(|i| {
        //     if devices[i].is_none() {
        //         devices[i] = Some(UartDevice {
        //             id: 0,
        //             default_console: false,
        //             driver: unsafe { &mut NS16550 },
        //         })
        //     }
        // });
    }
}
