use core::arch::global_asm;

// Call entry point from asm
global_asm!("
    .globl kstart
    kstart:
        j {start}
    ",
    start = sym start,
);

unsafe extern "C" fn start() -> ! {
    crate::main();
}

