use core::ptr;

use arrayvec::ArrayVec;

use crate::kprint;

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
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// Define a property header, len + nameoff of the prop follow by [u8;len] as the value of the prop
struct FdtPropHeader {
    len: u32,
    nameoff: u32,
}

// Structure to define a property parsed from fdt
#[derive(Clone, Copy)]
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
            // for _c in 0..node.prop_count {
            //     let current_prop: FdtPropHeader = props_buff[i];
                // let mut str_table_prop_name_off =
                //     string_block_off + current_prop.nameoff.swap_bytes() as usize;
            //     // Buff to store each char of the name
            //     let mut prop_name_buff: ArrayVec<u8, 31> = ArrayVec::new();
            //     // Loop and break when reaching end of the name str
            //     loop {
            //         let char =
            //             u8::from_be(unsafe { ptr::read(str_table_prop_name_off as *const u8) });
            //         if char == 0u8 {
            //             break;
            //         } else {
            //             prop_name_buff.push(char);
            //             str_table_prop_name_off += 1;
            //         }
            //     }
            //     // Get the value of the props from node.first_prop_off and assign it to the name
            //     let mut prop_value_buff: ArrayVec<u8, 556> = ArrayVec::new();
            //     let mut cursor = off_first_prop_struct + size_of::<FdtPropHeader>() as u32;
            //     for _ in 0..current_prop.len.swap_bytes() {
            //         let char = u8::from_be(unsafe { ptr::read(cursor as *const u8) });
            //         prop_value_buff.push(char);
            //         cursor += 1;
            //     }
            //     // Create static array and memcpy from ArrayVec buff
            //     let mut prop_name: [u8; 31] = [0u8; 31];
            //     prop_name.copy_from_slice(&prop_name_buff);
            //     let static_prop: Property = Property {
            //         name: prop_name,
            //         off_value: unsafe { PROPS_VALUE_MAX },
            //         value_len: prop_value_buff.len(),
            //     };
            //     unsafe {
            //         PROPERTIES_POOL[PROPS_COUNT] = static_prop;
            //         PROPS_COUNT += 1;
            //     }
            //     // Copy all content from prop_value_buff into the PROPS_VALUE_POOL
            //     unsafe {
            //         PROPS_VALUE_POOL[PROPS_VALUE_MAX..PROPS_VALUE_MAX + prop_value_buff.len()]
            //             .copy_from_slice(prop_value_buff.as_slice());
            //         // Increment the PROPS_VALUE_MAX to the size of the prop_value_buff to keep track
            //         // of the size of the pool and have correct offset
            //         PROPS_VALUE_MAX += prop_value_buff.len();
            //     };
            //     kprint!(
            //         "{}: {:?}\n",
            //         str::from_utf8(&prop_name_buff).unwrap(),
            //         prop_value_buff
            //     );
            //     // Increment prop_index to go to the next prop in the node
            //     i += 1;
            // }
            // Init driver for the node at the top of the stack
            continue;
        }
    }
}

pub fn get_all_fdt_nodes<'a>() -> &'a [FdtNode] {
    unsafe { &NODE_POOL[0..NODE_COUNT] }
}

pub fn get_fdt_node_prop<'a>(node: &FdtNode) -> &'a [Property] {
    let start = node.first_prop_off as usize;
    let end = node.first_prop_off + node.prop_count as u32;
    unsafe { &PROPERTIES_POOL[start..end as usize]}
}
