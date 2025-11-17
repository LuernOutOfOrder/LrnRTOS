use core::{
    fmt::{self, Write},
    ptr,
};

use arrayvec::ArrayVec;

use crate::{
    devices::{DriverRegion, serials::SERIAL_DEVICES},
    dtb::{
        FdtNode,
        helpers::{fdt_get_node_prop, fdt_get_node_prop_in_hierarchy},
    },
};

use super::{UartDevice, UartDriver};

/// Structure for Ns16550 driver
/// region: DriverRegion struct to define address memory region to use with the driver and the address size
#[derive(Copy, Clone)]
pub struct Ns16550 {
    pub region: DriverRegion,
}

/// Implementing the UartDriver trait for Ns16550 driver
impl UartDriver for Ns16550 {
    fn putchar(&self, c: u8) {
        unsafe { core::ptr::write_volatile(self.region.addr as *mut u8, c) }
    }
    fn getchar(&self) -> u8 {
        todo!()
    }
}

/// Implementing Write trait for Ns16550 to be able to format with core::fmt in print
/// Use the UartDriver function implemented in Ns16550
impl Write for Ns16550 {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            self.putchar(b);
        }
        Ok(())
    }
}

/// Static Ns16550 instance used when creating a new driver.
static mut NS16550_INSTANCE: Ns16550 = Ns16550 {
    region: DriverRegion { addr: 0, size: 0 },
};

/// Implementation of the Ns16550
impl Ns16550 {
    /// Init a new Ns16550 from the given fdt node
    pub fn init(node: &FdtNode) {
        // Get address and size cells
        let address_cells = fdt_get_node_prop_in_hierarchy(node, "#address-cells")
            .expect("ERROR: ns16550 node is missing '#address-cells' property from parent bus\n");
        let size_cells = fdt_get_node_prop_in_hierarchy(node, "#size-cells")
            .expect("ERROR: ns16550 node is missing '#size-cells' property from parent bus\n");
        // Ptr read address and size cells value from off and cast it to u32 target's endianness
        let address_cells_val: u32 =
            u32::from_be(unsafe { ptr::read(address_cells.off_value as *const u32) });
        let size_cells_val: u32 =
            u32::from_be(unsafe { ptr::read(size_cells.off_value as *const u32) });
        // Get device memory region
        let reg =
            fdt_get_node_prop(node, "reg").expect("ERROR: ns16550 node is missing 'reg' property");
        let mut reg_buff: ArrayVec<u32, 120> = ArrayVec::new();
        let mut reg_cursor = reg.off_value;
        // Divide reg.value_len by 4 because we read u32 and not u8
        for _ in 0..reg.value_len / 4 {
            let value = u32::from_be(unsafe { ptr::read(reg_cursor as *const u32) });
            reg_buff.push(value);
            reg_cursor += 4;
        }
        // Region size from #address-cells and #size-cells properties value
        let reg_size = address_cells_val + size_cells_val;
        // Init a new DriverRegion
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
        let ns16550: Ns16550 = Ns16550 {
            region: device_addr,
        };
        unsafe { NS16550_INSTANCE = ns16550 };
        let device = UartDevice {
            _id: 0,
            default_console: false,
            // Allow static mut refs because it's only use on early boot and there's no concurrent
            // access
            #[allow(static_mut_refs)]
            driver: unsafe { &mut NS16550_INSTANCE },
        };
        SERIAL_DEVICES.add_serial(device);
    }
}
