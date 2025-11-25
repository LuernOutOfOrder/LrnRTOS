use core::arch::global_asm;

use crate::kprint;

// Global asm for import start.S
global_asm!(include_str!("start.S"));

#[unsafe(no_mangle)]
unsafe extern "C" fn _start(_hartid: usize, dtb: usize) -> ! {
    kprint!("Really really early kprint\n");
    crate::main(dtb);
}
