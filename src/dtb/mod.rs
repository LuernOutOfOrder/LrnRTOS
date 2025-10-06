use core::ptr;

use crate::print::print_hex_u32;

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

pub fn parse_dtb_file(dtb: usize) {
    let header: DtbHeader = unsafe { ptr::read(dtb as *const DtbHeader) };
    print_hex_u32(0x10000000, header.magic.swap_bytes());
}
