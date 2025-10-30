use core::ptr;

use arrayvec::ArrayVec;

use crate::{
    dtb::{get_all_fdt_nodes, get_fdt_node_prop},
    kprint,
};

pub mod serials;

/// Init all drivers depending on nodes in fdt
pub fn init_devices() {
    // --- Nodes builder ---
    let fdt_nodes = get_all_fdt_nodes();
    // Main loop to build node from fdt pool
    for node in fdt_nodes {
        let props = get_fdt_node_prop(node);
        // ArrayVec of tuple to store all props names and values, avoid lifetime errors with slice
        let mut props_name: ArrayVec<(ArrayVec<u8, 31>, ArrayVec<u8, 512>), 16> = ArrayVec::new();
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
            props_name.push((prop_name, prop_value));
        }
        for each in props_name {
            kprint!(
                "prop name: {}\tprop value: {:?}\n",
                str::from_utf8(each.0.as_slice()).unwrap(),
                each.1
            );
        }
    }
}
