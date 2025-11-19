use core::arch::global_asm;

use crate::print;

global_asm!(
    "
    .global trap_entry
    .type trap_entry, @function
    trap_entry:
       csrr a0, mcause
       csrr a1, mepc
       call _trap_handler 
       mret
    "
);

#[unsafe(no_mangle)]
unsafe extern "C" fn _trap_handler(mcause: u32, mepc: usize) {
    // mcause strut -> u32 -> 31 bit = interrupt or exception.
    // if 31 bit is 1 -> interrupt, else 31 bit is 0 -> exception.
    // The remaining 30..0 bits is the interrupt or exception cause.
    // 31 bit[(interrupt or exception)] 30..0 bits[interrupt or exception cause]
    // Move all bits from mcause to 31 bits to the right to keep only the last bit
    // Last bit == interrupt
    let _interrupt = mcause >> 31;
    // Bit mask to keep all bits except the last bit
    let cause = mcause & 0x7FFFFFFF;
    match cause {
        7 => timer_interrupt(),
        _ => unimplemented!(),
    }
}

fn timer_interrupt() {
    panic!("EXIT");
}
