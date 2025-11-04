use core::arch::global_asm;

// Global asm for setting the sp, and jump to kernel entry point _start
global_asm!(
    "
    .section .text.entry
    .global kstart
    .type kstart, @function
    kstart:
        la sp, stack_top    # load address of stack_top
        j _start
    ",
);

#[unsafe(no_mangle)]
unsafe extern "C" fn _start(_hartid: usize, dtb: usize) -> ! {
    crate::main(dtb);
}
