use core::ptr;

use arrayvec::ArrayVec;

use crate::kprint;

static FDT_MAX_STACK: usize = 32;
static FDT_MAX_PROPS: usize = 1024;

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
#[derive(Copy, Clone)]
struct FdtNode {
    // Name is max 31 bytes + 0x00 to mark the end of the string
    name: [u8; 32],
    _parent_node_index: u32,
    // First prop offset used to find the first prop inside the structure block
    first_prop_off: u32,
    // Index of the first prop inside the props buffer
    first_prop_index: u16,
    prop_count: u16,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// Define a property header, len + nameoff of the prop follow by [u8;len] as the value of the prop
struct FdtPropHeader {
    len: u32,
    nameoff: u32,
}

pub fn parse_dtb_file(dtb: usize) {
    let header: FdtHeader = unsafe { ptr::read(dtb as *const FdtHeader) };
    // debug_print(0x10000000, "0x");
    if !header.valid_magic() {
        panic!("Magic from dtb is wrong");
    }
    let struct_block = dtb + header.off_dt_struct.swap_bytes() as usize;
    let string_block = dtb + header.off_dt_strings.swap_bytes() as usize;
    parse_dt_struct(struct_block, string_block);
}

#[unsafe(no_mangle)]
fn parse_dt_struct(dt_struct_addr: usize, _string_block_off: usize) {
    // Cursor to point the correct token inside the structure block
    let mut cursor = dt_struct_addr;
    // fdt token
    let fdt_begin_node = 0x00000001;
    let fdt_end_node = 0x00000002;
    let fdt_prop = 0x00000003;
    let fdt_nop = 0x4;
    let fdt_end = 0x9;

    // Nodes stack
    // Stack to save nodes in hierarchical order
    let mut node_stack: ArrayVec<FdtNode, FDT_MAX_STACK> = ArrayVec::new();
    // Buff to save node name
    let mut node_name: [u8; 32] = [0u8; 32];
    // Props buffer
    // Saves all props header
    let mut props_buff: ArrayVec<FdtPropHeader, FDT_MAX_PROPS> = ArrayVec::new();
    loop {
        let token = u32::from_be(unsafe { ptr::read(cursor as *const u32) });
        // Token to read each byte of the node name
        let mut node_name_token: u8;
        if token == fdt_begin_node {
            cursor += 4;
            for i in 0..31 {
                node_name_token = unsafe { ptr::read(cursor as *const u8) };
                // Break when reaching end of string
                if node_name_token == 0x00 {
                    node_name[i] = 0x00_u8;
                    break;
                }
                node_name[i] = node_name_token;

                // Increment the ptr to continue in the node name
                cursor += 1;
            }
            let node = FdtNode {
                name: node_name,
                _parent_node_index: 0,
                first_prop_off: 0,
                first_prop_index: 0,
                prop_count: 0,
            };
            // Push new node to top of the stack
            node_stack.push(node);
            // Reset node_name buff
            node_name = [0u8; 32];
            // bitwise to re align cursor on 4 bytes
            cursor = (cursor + 3) & !3;
            continue;
        }
        if token == fdt_nop {
            cursor += 4;
            continue;
        }
        if token == fdt_end {
            kprint!("Loop ended\n");
            break;
        }
        if token == fdt_prop {
            cursor += 4;
            if let Some(mut node) = node_stack.pop() {
                let prop_header: FdtPropHeader =
                    unsafe { ptr::read_unaligned(cursor as *const FdtPropHeader) };
                // kprint!("{:?}\n", prop_header);

                if node.first_prop_off == 0 {
                    node.first_prop_off = cursor as u32;
                    node.first_prop_index = props_buff.len() as u16;
                    node.prop_count += 1;
                }
                props_buff.push(prop_header);
                node_stack.push(node);
            }
            continue;
        }
        if token == fdt_end_node {
            // Pop top of the node stack
            // Init driver for the node at the top of the stack
            // let node = node_stack.pop().unwrap();
            let node = node_stack.last().unwrap();
            let node_name = str::from_utf8(&node.name).unwrap();
            kprint!("node: {:?}\n", node_name);
            cursor += 4;
            continue;
        }
        cursor += 4;
    }
}
