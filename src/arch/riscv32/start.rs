use core::arch::global_asm;

use crate::kprint;

// Global asm for import start.S
global_asm!(include_str!("start.S"));

#[unsafe(no_mangle)]
/// Kernel entry point for riscv32
unsafe extern "C" fn _start(hartid: usize, dtb: usize) -> ! {
    kprint!("Enter kernel RISC-V 32 bits entry point.\n");
    #[cfg(feature = "test")]
    crate::boot::test_kernel_early_boot(hartid, dtb);
    crate::boot::kernel_early_boot(hartid, dtb);
}
