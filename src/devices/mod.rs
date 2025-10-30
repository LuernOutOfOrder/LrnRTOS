use core::ptr;

use arrayvec::ArrayVec;
use serials::ns16550::Ns16550;

use crate::{
    dtb::{get_all_fdt_nodes, get_fdt_node_prop},
    kprint,
};

/// Module for serials devices
pub mod serials;

struct Driver<'a> {
    compatible: &'a str,
    init_fn: fn(&ArrayVec<u8, 31>, &ArrayVec<(ArrayVec<u8, 31>, ArrayVec<u8, 512>), 16>),
}

static DRIVERS: &[Driver] = &[Driver {
    compatible: "ns16550a",
    init_fn: Ns16550::init,
}];

/// Init all drivers depending on nodes in fdt
pub fn init_devices() {
    // --- Nodes builder ---
    let fdt_nodes = get_all_fdt_nodes();
    // Main loop to build node from fdt pool
    for node in fdt_nodes {
        let props = get_fdt_node_prop(node);
        // ArrayVec of tuple to store all props names and values, avoid lifetime errors with slice
        let mut props_buff: ArrayVec<(ArrayVec<u8, 31>, ArrayVec<u8, 512>), 16> = ArrayVec::new();
        // Fill props_buff with name and value for each props
        for prop in props {
            // Prop name
            let mut prop_name: ArrayVec<u8, 31> = ArrayVec::new();
            let mut nameoff_cursor = prop.nameoff;
            // Prop value
            let mut prop_value: ArrayVec<u8, 512> = ArrayVec::new();
            let mut value_cursor = prop.off_value;
            // Prop name loop
            for _ in 0..31 {
                let char = u8::from_be(unsafe { ptr::read(nameoff_cursor as *const u8) });
                if char == 0u8 {
                    break;
                } else {
                    prop_name.push(char);
                    nameoff_cursor += 1;
                }
            }
            // Prop value loop
            for _ in 0..prop.value_len {
                let token = u8::from_be(unsafe { ptr::read(value_cursor as *const u8) });
                prop_value.push(token);
                value_cursor += 1;
            }
            props_buff.push((prop_name, prop_value));
        }
        // Loop for node name
        let mut node_name_cursor = node.nameoff;
        let mut node_name_buff: ArrayVec<u8, 31> = ArrayVec::new();
        for _ in 0..31 {
            let char = u8::from_be(unsafe { ptr::read(node_name_cursor as *const u8) });
            if char == 0u8 {
                break;
            } else {
                node_name_buff.push(char);
            }
            node_name_cursor += 1;
        }
        // Iter over all props and match a compatible drivers, then call the init fn with nodes and
        // props
        for props in &props_buff {
            let str = str::from_utf8(&props.0).unwrap();
            if str == "compatible" {
                let compatible = str::from_utf8(&props.1).unwrap().trim_end_matches('\0');
                for driver in DRIVERS {
                    if compatible == driver.compatible {
                        (driver.init_fn)(&node_name_buff, &props_buff)
                    }
                }
            }
        }
    }
}
