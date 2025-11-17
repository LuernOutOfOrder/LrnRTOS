use core::ptr;

use arrayvec::ArrayVec;

use super::{FdtNode, NODE_COUNT, NODE_POOL, PROPERTIES_POOL, Property};

/// Return slice from NODE_POOL with correct len
pub fn fdt_get_all_nodes<'a>() -> &'a [FdtNode] {
    unsafe { &NODE_POOL[0..NODE_COUNT] }
}

/// Return the node from given index in the NODE_POOL
pub fn fdt_get_node(index: usize) -> FdtNode {
    unsafe { NODE_POOL[index] }
}

/// Return the index of the given node in the NODE_POOL
pub fn fdt_get_index_from_node(node: &FdtNode) -> usize {
    #[allow(clippy::needless_range_loop)]
    for i in 0..unsafe { NODE_COUNT } {
        let current = unsafe { NODE_POOL[i] };
        if current.first_prop_off == node.first_prop_off {
            return i;
        }
    }
    0
}

/// Return all properties from given node as a slice of Property
pub fn fdt_get_all_node_props<'a>(node: &FdtNode) -> &'a [Property] {
    let start = node.first_prop_off as usize;
    let end = node.first_prop_off + node.prop_count as u32;
    unsafe { &PROPERTIES_POOL[start..end as usize] }
}

/// Get wanted prop from given node
/// Return an Option<Property>, caller have to make the parsing from fdt with Property field
/// Return None if no prop was found in given node.
/// node: the node to search the wanted property in.
/// prop_name: the wanted property to find.
pub fn fdt_get_node_prop(node: &FdtNode, prop_name: &str) -> Option<Property> {
    let props = fdt_get_all_node_props(node);
    for prop in props {
        // Prop name
        let mut prop_name_buff: ArrayVec<u8, 31> = ArrayVec::new();
        let mut nameoff_cursor = prop.nameoff;
        // Prop name loop
        for _ in 0..31 {
            let char = u8::from_be(unsafe { ptr::read(nameoff_cursor as *const u8) });
            if char == 0u8 {
                break;
            } else {
                prop_name_buff.push(char);
                nameoff_cursor += 1;
            }
        }
        let prop_name_str = str::from_utf8(&prop_name_buff)
            .expect("Failed to cast &[u8] to &str. Invalid UTF-8 char in FDT property")
            .trim_end_matches('\0');
        // Check prop name with wanted prop_name
        if prop_name_str == prop_name {
            return Some(*prop);
        }
    }
    None
}

/// Get wanted prop from given node, if no one is found, check prop in parent node. Same logic as
/// get_node_prop function but with hierarchy logic.
/// Return None if no prop was found in given node.
/// node: the node to search the wanted property in.
/// prop_name: the wanted property to find.
pub fn fdt_get_node_prop_in_hierarchy(node: &FdtNode, prop_name: &str) -> Option<Property> {
    // Use index from node instead of node to avoid lifetime issue
    let mut current_search_node = fdt_get_index_from_node(node);
    // Loop to check props in given node, if asked prop is not found, check in parent node, etc.
    // Number of node to iterate over
    for _ in 0..2 {
        let search_node = fdt_get_node(current_search_node);
        let props = fdt_get_all_node_props(&search_node);
        // Iter over props to find the wanted props
        for prop in props {
            // Prop name
            let mut prop_name_buff: ArrayVec<u8, 31> = ArrayVec::new();
            let mut nameoff_cursor = prop.nameoff;
            // Prop name loop
            for _ in 0..31 {
                let char = u8::from_be(unsafe { ptr::read(nameoff_cursor as *const u8) });
                if char == 0u8 {
                    break;
                } else {
                    prop_name_buff.push(char);
                    nameoff_cursor += 1;
                }
            }
            let prop_name_str = str::from_utf8(&prop_name_buff)
                .expect("Failed to cast &[u8] to &str. Invalid UTF-8 char in FDT property")
                .trim_end_matches('\0');
            // Check prop name with wanted prop_name
            if prop_name_str == prop_name {
                return Some(*prop);
            }
        }
        // If reaching this point, it means that the wanted prop was not found in current node, so
        // update the search_node to be the parent one
        current_search_node = search_node
            .parent_node_index
            .expect("Failed to get the FDT parent node");
    }
    None
}

/// Iterate over all nodes and return the node containing the wanted phandle
pub fn fdt_get_node_by_phandle(phandle: u32) -> Option<FdtNode> {
    let nodes = fdt_get_all_nodes();
    for node in nodes {
        if let Some(node_phandle) = fdt_get_node_prop(node, "phandle") {
            let phandle_value =
                u32::from_be(unsafe { ptr::read(node_phandle.off_value as *const u32) });
            if phandle_value == phandle {
                return Some(*node);
            }
        } else {
            continue;
        }
    }
    None
}

pub fn fdt_get_node_name(node: &FdtNode) -> ArrayVec<u8, 31> {
    let mut node_name: ArrayVec<u8, 31> = ArrayVec::new();
    let mut off = node.nameoff;
    for _ in 0..31 {
        let char = u8::from_be(unsafe { ptr::read(off as *const u8) });
        if char == 0u8 {
            break;
        } else {
            node_name.push(char);
            off += 1;
        }
    }
    node_name
}

/// Return node from name
pub fn fdt_get_node_by_name(name: &str) -> Option<&FdtNode> {
    let nodes = fdt_get_all_nodes();
    for node in nodes {
        let node_name = fdt_get_node_name(node);
        let node_name_str: &str =
            str::from_utf8(&node_name).expect("Failed to cast node name to &str");
        if node_name_str == name {
            return Some(node);
        }
    }
    None
}

/// Return prop from given node name and prop name
pub fn fdt_get_prop_by_node_name(node_name: &str, prop_name: &str) -> Option<Property> {
    let node = fdt_get_node_by_name(node_name).expect("Failed to get node from node name");
    fdt_get_node_prop(node, prop_name)
}

/// Return an u32 value from given fdt property
pub fn fdt_get_prop_u32_value(prop: Property) -> u32 {
    u32::from_be(unsafe { ptr::read(prop.off_value as *const u32) })
}
