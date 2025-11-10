use core::ptr;

use arrayvec::ArrayVec;

use crate::{
    devices::DriverRegion,
    dtb::{
        FdtNode,
        helpers::{get_node_prop, get_node_prop_in_hierarchy},
    },
};

pub struct Clint0 {
    region: DriverRegion,
    interrupt_extended: [u32; 8],
}

impl Clint0 {
    pub fn init(node: &FdtNode) {
        // Get address and size cells
        let address_cells = get_node_prop_in_hierarchy(node, "#address-cells")
            .expect("ERROR: clint0 node is missing '#address-cells' property from parent bus\n");
        let size_cells = get_node_prop_in_hierarchy(node, "#size-cells")
            .expect("ERROR: clint0 node is missing '#size-cells' property from parent bus\n");
        // Ptr read address and size cells value from off and cast it to u32 target's endianness
        let address_cells_val: u32 =
            u32::from_be(unsafe { ptr::read(address_cells.off_value as *const u32) });
        let size_cells_val: u32 =
            u32::from_be(unsafe { ptr::read(size_cells.off_value as *const u32) });
        // Get device memory region
        let reg = get_node_prop(node, "reg").expect("ERROR: clint0 node is missing 'reg' property");
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
        let interrupt_extended_array: [u32; 8] = [0u32; 8];
        let interrupt_extended = get_node_prop(node, "interrupt-extended")
            .expect("ERROR: clint0 node is missing 'interrupt-extended' property\n");
        let clint0: Clint0 = Clint0 {
            region: device_addr,
            interrupt_extended: interrupt_extended_array,
        };
    }
}
