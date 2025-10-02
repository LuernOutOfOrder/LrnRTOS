use core::arch::global_asm;

// Call entry point from asm
global_asm!("
    .globl kstart
    kstart:
        j {start}
    ",
    start = sym _start,
);

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.entry")]
unsafe extern "C" fn _start() -> ! {
    // let dtb: u32 = unsafe { core::ptr::read_volatile(0x11 as *const u32) };
    crate::main(0);
}
