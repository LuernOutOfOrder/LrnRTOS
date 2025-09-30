use core::arch::global_asm;

// Call entry point from asm
global_asm!("
    .globl kstart
    kstart:
        j {start}
    ",
    start = sym _start,
);

#[no_mangle]
#[link_section = ".text.entry"]
unsafe extern "C" fn _start() -> ! {
    crate::main();
}

