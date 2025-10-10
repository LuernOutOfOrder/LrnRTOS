use core::ptr;

use arrayvec::ArrayVec;

use crate::print::{debug_print, print_hex_u32};

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
    name: usize,
    _parent_node_index: u32,
    // First prop offset used to find the first prop inside the structure block
    first_prop_off: u32,
    // Index of the first prop inside the props buffer
    first_prop_index: u16,
    prop_count: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
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
    parse_dt_struct(struct_block, dtb);
}

#[unsafe(no_mangle)]
fn parse_dt_struct(dt_struct_addr: usize, base_addr: usize) {
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

    // Props buffer
    // Saves all props header
    let mut props_buff: ArrayVec<FdtPropHeader, FDT_MAX_PROPS> = ArrayVec::new();
    loop {
        let token = u32::to_be(unsafe { ptr::read(cursor as *const u32) });
        print_hex_u32(token);
        debug_print("\n");
        if token == fdt_begin_node {
            debug_print("debug begin_node\n");
            cursor += 4;
            let node = FdtNode {
                name: cursor,
                _parent_node_index: 0,
                first_prop_off: 0,
                first_prop_index: 0,
                prop_count: 0,
            };
            // Push new node to top of the stack
            node_stack.push(node);
            continue;
        }
        if token == fdt_nop {
            cursor += 4;
            continue;
        }
        if token == fdt_end {
            debug_print("Loop ended");
            break;
        }
        if token == fdt_prop {
            cursor += 4;
            // let node: FdtNode = unsafe { NODE_STACK[stack_index] };
            // debug_print(0x10000000, "debug\n");
            // print_hex_u32(0x10000000, cursor as u32);
            // debug_print(0x10000000, "\n");
            // print_hex_u32(0x10000000, node.name as u32);
            // let property: FdtPropHeader = unsafe { ptr::read(cursor as *const FdtPropHeader) };

            continue;
        }
        if token == fdt_end_node {
            debug_print("debug end_node\t");
            // Pop top of the node stack
            node_stack.pop();
            cursor += 4;
            continue;
        }
        // match token {
        //     fdt_nop => {
        //         cursor += 4;
        //     }
        //     fdt_end => {
        //         debug_print(0x10000000, "loop ended");
        //         break;
        //     }
        //     _ => cursor += 4,
        // }
        cursor += 4;
    }
}
