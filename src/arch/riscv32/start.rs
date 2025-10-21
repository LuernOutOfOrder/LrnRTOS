use core::{arch::global_asm, fmt::Write};

use crate::{devices::serials::set_kconsole, log::BootWriter, print::write_fmt};

// Call entry point from asm
global_asm!(
    "
    .section .text.entry
    .global kstart
    .type kstart, @function
    kstart:
        la sp, __stack_top    # load address of __stack_top
        j _start              # jump to your Rust _start
    ",
);

#[unsafe(no_mangle)]
unsafe extern "C" fn _start(_hartid: usize, dtb: usize) -> ! {
    static mut EARLY_WRITER: BootWriter = BootWriter {
        base_addr: 0x1000_0000 as *mut u8,
    };
    set_kconsole(unsafe { &mut EARLY_WRITER });
    crate::main(dtb);
}
