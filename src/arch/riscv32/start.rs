use core::arch::global_asm;

// Global asm for import start.S 
global_asm!(include_str!("start.S"));

#[unsafe(no_mangle)]
unsafe extern "C" fn _start(_hartid: usize, dtb: usize) -> ! {
    // static mut EARLY_WRITER: BootWriter = BootWriter {
    //     base_addr: 0x1000_0000 as *mut u8,
    // };
    // #[allow(static_mut_refs)]
    // KCONSOLE.set(unsafe { &mut EARLY_WRITER });
    crate::main(dtb);
}
