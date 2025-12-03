use core::ptr;

use arrayvec::ArrayVec;

use crate::config::{FDT_MAX_PROPS, FDT_MAX_STACK};

// Helpers module for node's props recovery
pub mod helpers;

/// Structure for the fdt header, used for parsing fdt. Based on the given structure in official
/// device tree specifications. See: https://devicetree-specification.readthedocs.io/en/stable/
#[repr(C)]
#[derive(Copy, Clone)]
struct FdtHeader {
    magic: u32,
    total_size: u32,
    off_dt_struct: u32,
    off_dt_strings: u32,
    off_mem_rsvmap: u32,
    version: u32,
    last_comp_version: u32,
    boot_cpuid_phys: u32,
    size_dt_strings: u32,
    size_dt_struct: u32,
}

/// Implementation for the FdtHeader to easily check magic number of other thing if needed later
impl FdtHeader {
    fn valid_magic(&self) -> bool {
        self.magic.swap_bytes() == 0xd00dfeed
    }

    fn _struct_range(&self) -> core::ops::Range<usize> {
        let start = self.off_dt_struct as usize;
        let end = start + self.size_dt_struct as usize;

        start..end
    }
}

/// Definition of a node, used to save node information in static pool for node recovery outside
/// the fdt parsing.
/// nameoff: offset to the node name in structure block.
/// first_prop_off: offset to the first node's prop in the PROPERTIES_POOL, save only the
/// first property because all node's properties are following each other in the structure block. So we
/// only need the first property and a counter of properties to recover them all.
/// prop_count: counter to keep track of all property found. Increment each time a new property is
/// found
/// parent_node_index: the index of the parent node in the device tree, index in NODE_POOL.
/// Important for keeping the hierarchy of the device tree.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FdtNode {
    // Name is max 31 bytes
    pub nameoff: u32,
    pub first_prop_off: u32,
    pub prop_count: u16,
    pub parent_node_index: Option<usize>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// Define a property header, len + nameoff of the prop follow by [u8;len] as the value of the
/// property
struct FdtPropHeader {
    len: u32,
    nameoff: u32,
}

/// Structure to define a property parsed from fdt, used to save property information in static
/// pool for property recovery outside the fdt parsing.
/// nameoff: offset to the property name in the string block.
/// off_value: offset to the property value in the structure block.
/// value_len: size of the value in the structure block. Used for parsing and getting the correct
/// value size.
#[derive(Clone, Copy, Debug)]
pub struct Property {
    pub nameoff: usize,
    pub off_value: usize,
    pub value_len: u32,
}

// Static to save all parsed node. Used in helpers functions
static mut NODE_POOL: [FdtNode; FDT_MAX_STACK] = [FdtNode {
    nameoff: 0,
    first_prop_off: 0,
    prop_count: 0,
    parent_node_index: None,
}; FDT_MAX_STACK];

// Static to save all parsed properties, use the node.first_prop..node.first_prop + node.prop_count
// to get all properties for a specific node.
static mut PROPERTIES_POOL: [Property; FDT_MAX_PROPS] = [Property {
    nameoff: 0,
    off_value: 0,
    value_len: 0,
}; FDT_MAX_PROPS];

// Node and props count to iterate over the static pools. Also used to point to the correct max size of the
// pool, not the len of the pool.
// Note: use maybeuninit for the pool later?
static mut NODE_COUNT: usize = 0;
static mut PROPS_COUNT: usize = 0;

/// Parse the dtb header from the given address and call structure block parsing function
pub fn parse_dtb_file(dtb: usize) {
    let header: FdtHeader = unsafe { ptr::read(dtb as *const FdtHeader) };
    if !header.valid_magic() {
        panic!("Magic from dtb is wrong");
    }
    // Offset to the structure block and string block
    let struct_block = dtb + header.off_dt_struct.swap_bytes() as usize;
    let string_block = dtb + header.off_dt_strings.swap_bytes() as usize;
    parse_fdt_struct(struct_block, string_block);
}

/// Parse the structure block and save all node and properties most important data in static pool.
/// dt_struct_addr: offset to the structure block where all nodes and properties data is define.
/// string_block_off: offset to the string_block_off where all properties name is define. Only used
/// for saving property name offset in structure
fn parse_fdt_struct(dt_struct_addr: usize, string_block_off: usize) {
    // Cursor to point the correct token inside the structure block
    let mut cursor = dt_struct_addr;
    // fdt token
    let fdt_begin_node = 0x00000001;
    let fdt_end_node = 0x00000002;
    let fdt_prop = 0x00000003;
    let fdt_nop = 0x00000004;
    let fdt_end = 0x00000009;

    // Stack used to save NODE_POOL size and keep hierarchie during the parsing
    let mut node_stack: ArrayVec<usize, FDT_MAX_STACK> = ArrayVec::new();
    loop {
        let token = u32::from_be(unsafe { ptr::read(cursor as *const u32) });
        cursor += 4;
        if token == fdt_begin_node {
            let node = FdtNode {
                nameoff: cursor as u32,
                first_prop_off: 0,
                prop_count: 0,
                parent_node_index: {
                    if node_stack.is_empty() {
                        None
                    } else {
                        // Parent index is the last element of the stack (index inside the
                        // NODE_POOL)
                        Some(*node_stack.last().unwrap())
                    }
                },
            };
            // Push new node index to top of the stack
            node_stack.push(unsafe { NODE_COUNT });
            unsafe {
                NODE_POOL[NODE_COUNT] = node;
                // Increment node_count
                NODE_COUNT += 1;
            };
            // Bitwise to re align cursor on 4 bytes
            cursor = (cursor + 3) & !3;
            continue;
        }
        if token == fdt_nop {
            continue;
        }
        if token == fdt_end {
            break;
        }
        if token == fdt_prop {
            // Cast current cursor ptr as prop header
            let prop_header: FdtPropHeader = unsafe { ptr::read(cursor as *const FdtPropHeader) };
            let idx = node_stack.last().unwrap();
            let mut node = unsafe { NODE_POOL[*idx] };
            if node.first_prop_off == 0 {
                node.first_prop_off = unsafe { PROPS_COUNT } as u32;
                node.prop_count += 1;
            } else {
                node.prop_count += 1;
            }
            let prop: Property = Property {
                nameoff: string_block_off + prop_header.nameoff.swap_bytes() as usize,
                off_value: cursor + size_of::<FdtPropHeader>(),
                value_len: prop_header.len.swap_bytes(),
            };
            // Push new property in static pool and increment static pool prop counter
            unsafe {
                PROPERTIES_POOL[PROPS_COUNT] = prop;
                PROPS_COUNT += 1;
            }
            // Update node from NODE_POOL
            unsafe { NODE_POOL[*idx] = node };
            // Increment the cursor by the len of the prop
            cursor += size_of::<FdtPropHeader>() + prop_header.len.swap_bytes() as usize;
            // Align cursor on 4 bytes
            cursor = (cursor + 3) & !3;
            continue;
        }
        if token == fdt_end_node {
            // Pop top of the node stack or continue if stack empty
            if !node_stack.is_empty() {
                node_stack
                    .pop()
                    .expect("Failed to pop the top of FDT node stack");
            } else {
                continue;
            }
            continue;
        }
    }
}
