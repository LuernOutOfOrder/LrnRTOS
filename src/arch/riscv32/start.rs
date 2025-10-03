use core::arch::global_asm;

// Call entry point from asm
global_asm!("
    .globl kstart
    kstart:
        la sp, STACK_TOP
        j _start
    ",
);

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.entry")]
unsafe extern "C" fn _start(temp: usize, dtb_ptr: usize) -> ! {
    let dtb: u32 = unsafe { core::ptr::read_volatile(dtb_ptr as *const u32) };
    let dtb_addr: *const u8 = dtb as *const u8;
    crate::main(dtb_addr);
}
