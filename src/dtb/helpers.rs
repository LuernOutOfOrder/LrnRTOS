use core::ptr;

use arrayvec::ArrayVec;

use super::{FdtNode, NODE_COUNT, NODE_POOL, PROPERTIES_POOL, Property};

/// Return slice from NODE_POOL with correct len
pub fn get_all_fdt_nodes<'a>() -> &'a [FdtNode] {
    unsafe { &NODE_POOL[0..NODE_COUNT] }
}

/// Return node from given index
pub fn get_fdt_node(index: usize) -> FdtNode {
    unsafe { NODE_POOL[index] }
}

/// Return the index of the given node in the node pool
pub fn get_index_fdt_node(node: &FdtNode) -> usize {
    for i in 0..unsafe { NODE_COUNT } {
        let current = unsafe { NODE_POOL[i] };
        if current.first_prop_off == node.first_prop_off {
            return i;
        }
    }
    0
}

/// Return a slice of props from given node
pub fn get_fdt_node_prop<'a>(node: &FdtNode) -> &'a [Property] {
    let start = node.first_prop_off as usize;
    let end = node.first_prop_off + node.prop_count as u32;
    unsafe { &PROPERTIES_POOL[start..end as usize] }
}

/// Get wanted prop from given node
/// Return an Option<Property>, caller have to make the parsing from fdt with Property field
/// Return None if no prop was found in given node
pub fn get_node_prop(node: &FdtNode, prop_name: &str) -> Option<Property> {
    let props = get_fdt_node_prop(node);
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
            .unwrap()
            .trim_end_matches('\0');
        // Check prop name with wanted prop_name
        if prop_name_str == prop_name {
            return Some(*prop);
        }
    }
    None
}

/// Get wanted prop from given node, if no one is found, check prop in parent node
pub fn get_node_prop_in_hierarchy(node: &FdtNode, prop_name: &str) -> Option<Property> {
    // Use index from node instead of node to avoid lifetime issue
    let mut current_search_node = get_index_fdt_node(node);
    // Loop to check props in given node, if asked prop is not found, check in parent node, etc.
    // Number of node to iterate over
    for _ in 0..2 {
        let search_node = get_fdt_node(current_search_node);
        let props = get_fdt_node_prop(&search_node);
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
                .unwrap()
                .trim_end_matches('\0');
            // Check prop name with wanted prop_name
            if prop_name_str == prop_name {
                return Some(*prop);
            }
        }
        // If reaching this point, it means that the wanted prop was not found in current node, so
        // update the search_node to be the parent one
        current_search_node = search_node.parent_node_index.unwrap();
    }
    None
}
