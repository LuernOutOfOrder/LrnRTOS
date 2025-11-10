use core::ptr;

use arrayvec::ArrayVec;
use serials::ns16550::Ns16550;

use crate::dtb::{
    FdtNode,
    helpers::{get_all_fdt_nodes, get_node_prop},
};

/// Module for serials devices
pub mod serials;

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
#[derive(Copy, Clone)]
pub struct DriverRegion {
    pub addr: usize,
    pub size: usize,
}

/// Static array to save all handled drivers in the kernel. Point to the init function used to init
/// the driver.
static DRIVERS: &[Driver] = &[Driver {
    compatible: "ns16550a",
    init_fn: Ns16550::init,
}];

/// Init all drivers, get all nodes parsed from fdt, and check compatible field. Pass the node to
/// the corresponding driver init_fn.
pub fn init_devices() {
    // Get all nodes
    let fdt_nodes = get_all_fdt_nodes();
    // Loop used to check compatible prop on each node, check compatible value and call
    // corresponding driver
    for node in fdt_nodes {
        // Get compatible prop from node
        if let Some(prop) = get_node_prop(node, "compatible") {
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
