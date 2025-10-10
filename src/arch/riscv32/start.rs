use core::arch::global_asm;

use crate::{devices::serials::set_kconsole, log::BootWriter};

// Call entry point from asm
global_asm!(
    "
    .globl kstart
    kstart:
        la sp, stack_top
        j _start
    ",
);

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.entry")]
unsafe extern "C" fn _start(_hartid: usize, dtb: usize) -> ! {
    static mut EARLY_WRITER: BootWriter = BootWriter {
        base_addr: 0x1000_0000 as *mut u8,
    };
    set_kconsole(unsafe { &mut EARLY_WRITER });
    crate::main(dtb);
}
