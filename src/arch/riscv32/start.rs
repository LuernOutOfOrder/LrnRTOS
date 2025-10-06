use core::arch::global_asm;

// Call entry point from asm
global_asm!(
    "
    .globl kstart
    kstart:
        la sp, _stack_top
        j _start
    ",
);

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.entry")]
unsafe extern "C" fn _start(_hartid: usize, dtb: usize) -> ! {
    crate::main(dtb);
}
