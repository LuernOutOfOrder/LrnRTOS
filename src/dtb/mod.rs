use core::ptr;

use arrayvec::ArrayVec;

use crate::kprint;

// Helpers module for node's props recovery
pub mod helpers;

static FDT_MAX_STACK: usize = 64;
static FDT_MAX_PROPS: usize = 128;

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

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FdtNode {
    // Name is max 31 bytes
    pub nameoff: u32,
    // First prop offset used to find the first prop inside prop pool
    pub first_prop_off: u32,
    pub prop_count: u16,
    // Parent node representing by the index in the stack or the static array in mem
    pub parent_node_index: Option<usize>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// Define a property header, len + nameoff of the prop follow by [u8;len] as the value of the prop
struct FdtPropHeader {
    len: u32,
    nameoff: u32,
}

// Structure to define a property parsed from fdt
#[derive(Clone, Copy, Debug)]
pub struct Property {
    pub nameoff: usize,
    pub off_value: usize,
    pub value_len: u32,
}

// Static to save all parsed node
static mut NODE_POOL: [FdtNode; FDT_MAX_STACK] = [FdtNode {
    nameoff: 0,
    first_prop_off: 0,
    prop_count: 0,
    parent_node_index: None,
}; FDT_MAX_STACK];

// Static to save all parsed properties, use the node.first_prop..node.first_prop + node.prop_count
// to get all props for a specific node.
static mut PROPERTIES_POOL: [Property; FDT_MAX_PROPS] = [Property {
    nameoff: 0,
    off_value: 0,
    value_len: 0,
}; FDT_MAX_PROPS];

// Node and props count to iterate over the static pools
static mut NODE_COUNT: usize = 0;
static mut PROPS_COUNT: usize = 0;

// Parse the dtb header and call other parsing functions
pub fn parse_dtb_file(dtb: usize) {
    let header: FdtHeader = unsafe { ptr::read(dtb as *const FdtHeader) };
    if !header.valid_magic() {
        panic!("Magic from dtb is wrong");
    }
    let struct_block = dtb + header.off_dt_struct.swap_bytes() as usize;
    let string_block = dtb + header.off_dt_strings.swap_bytes() as usize;
    parse_fdt_struct(struct_block, string_block);
}

// Parse the structure block and save all node and prop header in buff
fn parse_fdt_struct(dt_struct_addr: usize, string_block_off: usize) {
    // Cursor to point the correct token inside the structure block
    let mut cursor = dt_struct_addr;
    // fdt token
    let fdt_begin_node = 0x00000001;
    let fdt_end_node = 0x00000002;
    let fdt_prop = 0x00000003;
    let fdt_nop = 0x4;
    let fdt_end = 0x9;

    let mut node_stack: ArrayVec<FdtNode, FDT_MAX_STACK> = ArrayVec::new();
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
                        Some(node_stack.len())
                    }
                },
            };
            // Push new node to top of the stack
            node_stack.push(node);
            // Bitwise to re align cursor on 4 bytes
            cursor = (cursor + 3) & !3;
            continue;
        }
        if token == fdt_nop {
            continue;
        }
        if token == fdt_end {
            kprint!("Loop ended\n");
            break;
        }
        if token == fdt_prop {
            // Cast current cursor ptr as prop header
            let prop_header: FdtPropHeader = unsafe { ptr::read(cursor as *const FdtPropHeader) };
            if let Some(mut node) = node_stack.pop() {
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
                unsafe {
                    PROPERTIES_POOL[PROPS_COUNT] = prop;
                    PROPS_COUNT += 1;
                }
                node_stack.push(node);
            }
            // Increment the cursor by the len of the prop
            cursor += size_of::<FdtPropHeader>() + prop_header.len.swap_bytes() as usize;
            // Align cursor on 4 bytes
            cursor = (cursor + 3) & !3;
            continue;
        }
        if token == fdt_end_node {
            // Pop top of the node stack or continue if stack empty
            let node = {
                if !node_stack.is_empty() {
                    node_stack.pop().unwrap()
                } else {
                    continue;
                }
            };
            // Update node pool to add static node
            unsafe {
                NODE_POOL[NODE_COUNT] = node;
                // Increment node_count
                NODE_COUNT += 1;
            };
            continue;
        }
    }
}

