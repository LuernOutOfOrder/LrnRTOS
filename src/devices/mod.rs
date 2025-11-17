use core::ptr;

use arrayvec::ArrayVec;
use serials::ns16550::Ns16550;

use crate::dtb::{
    FdtNode,
    helpers::{fdt_get_all_nodes, fdt_get_node_prop, fdt_get_node_prop_in_hierarchy},
};

/// Module for serials devices
pub mod serials;

// Module for timer devices
pub mod timer;

// Module for cpu core interrupt-controller
pub mod cpu_intc;

// Module for cpu frequency
pub mod cpufreq;

/// Structure used to define a Driver for compatible matching.
/// Only used in static DRIVERS
/// compatible: name of the compatible driver for this device.
/// init_fn: function to init the driver with given node from fdt parsing.
struct Driver<'a> {
    compatible: &'a str,
    init_fn: fn(&FdtNode),
}

/// Public structure used to define device region in memory.
/// addr: the address to use in drivers.
/// size: the size of the address.
#[derive(Copy, Clone, Debug)]
pub struct DriverRegion {
    pub addr: usize,
    pub size: usize,
}

impl DriverRegion {
    pub fn new(node: &FdtNode) -> Self {
        // Get address and size cells
        let address_cells = fdt_get_node_prop_in_hierarchy(node, "#address-cells")
            .expect("ERROR: node is missing '#address-cells' property from parent bus\n");
        let size_cells = fdt_get_node_prop_in_hierarchy(node, "#size-cells")
            .expect("ERROR: node is missing '#size-cells' property from parent bus\n");
        // Ptr read address and size cells value from off and cast it to u32 target's endianness
        let address_cells_val: u32 =
            u32::from_be(unsafe { ptr::read(address_cells.off_value as *const u32) });
        let size_cells_val: u32 =
            u32::from_be(unsafe { ptr::read(size_cells.off_value as *const u32) });
        // Get device memory region
        let reg = fdt_get_node_prop(node, "reg").expect("ERROR: node is missing 'reg' property");
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
        device_addr
    }
}

/// Static driver match table to save all handled drivers in the kernel. Point to the init function used to init
/// the driver.
static DRIVERS: &[Driver] = &[
    Driver {
        compatible: "ns16550a",
        init_fn: Ns16550::init,
    },
    Driver {
        compatible: "sifive,clint0",
        init_fn: timer::clint0::Clint0::init,
    },
    Driver {
        compatible: "riscv,cpu-intc",
        init_fn: cpu_intc::riscv_cpu_intc::CpuIntc::init,
    },
];

/// Init all drivers, get all nodes parsed from fdt, and check compatible field. Pass the node to
/// the corresponding driver init_fn.
pub fn init_devices() {
    // Get all nodes
    let fdt_nodes = fdt_get_all_nodes();
    // Loop used to check compatible prop on each node, check compatible value and call
    // corresponding driver
    for node in fdt_nodes {
        // Get compatible prop from node
        if let Some(prop) = fdt_get_node_prop(node, "compatible") {
            // Get the value of compatible property.
            let mut prop_value_buff: ArrayVec<u8, 32> = ArrayVec::new();
            let mut prop_value_cursor = prop.off_value;
            for _ in 0..prop.value_len {
                let char = u8::from_be(unsafe { ptr::read(prop_value_cursor as *const u8) });
                if char == 0u8 {
                    break;
                } else {
                    prop_value_buff.push(char);
                    prop_value_cursor += 1;
                }
            }
            // Check props and match a compatible drivers, then call the init fn with nodes
            let str = str::from_utf8(&prop_value_buff)
                .expect("Failed to cast &[u8] to &str. Invalid UTF-8 char in FDT property")
                .trim_end_matches('\0');
            for driver in DRIVERS {
                if str == driver.compatible {
                    (driver.init_fn)(node)
                }
            }
        } else {
            continue;
        }
    }
}
