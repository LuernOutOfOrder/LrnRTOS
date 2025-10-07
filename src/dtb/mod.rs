use core::ptr;

use crate::print::{debug_print, print_hex_u32};

#[repr(C)]
#[derive(Copy, Clone)]
struct DtbHeader {
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

impl DtbHeader {
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
struct _FdtProp {
    len: u32,
    nameoff: u32,
}

pub fn parse_dtb_file(dtb: usize) {
    let header: DtbHeader = unsafe { ptr::read(dtb as *const DtbHeader) };
    // debug_print(0x10000000, "0x");
    if !header.valid_magic() {
        panic!("Magic from dtb is wrong");
    }
    let struct_block = dtb + header.off_dt_struct.swap_bytes() as usize;
    parse_dt_struct(struct_block);
}

fn parse_dt_struct(dt_struct_addr: usize) {
    let mut cursor = dt_struct_addr;
    let fdt_begin_node = 0x00000001;
    let fdt_end_node = 0x00000002;
    let fdt_prop = 0x00000003;
    let fdt_nop = 0x00000004;
    let fdt_end = 0x00000009;
    loop {
        let token = u32::to_be(unsafe { ptr::read(cursor as *const u32) });
        print_hex_u32(0x10000000, token);
        if token == fdt_nop {
            cursor += 4;
            continue;
        }
        if token == fdt_end {
            debug_print(0x10000000, "Loop ended");
            break;
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
