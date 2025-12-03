use core::arch::global_asm;

use crate::kprint;

// Global asm for import start.S
global_asm!(include_str!("start.S"));

#[unsafe(no_mangle)]
/// Kernel entry point for riscv32
unsafe extern "C" fn _start(_hartid: usize, dtb: usize) -> ! {
    kprint!("Enter kernel entry point\n");
    crate::main(dtb);
}
