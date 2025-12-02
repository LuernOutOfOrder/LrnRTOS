use core::arch::global_asm;

use crate::{
    arch::TrapFrame,
    config::TICK_DURATION,
    ktime::{set_ktime_ms, tick::increment_tick},
};

// Include gnu_macro asm file in compilation
global_asm!(include_str!("gnu_macro.S"));
// Include trap_entry asm file for trap entry fn in compilation
global_asm!(include_str!("trap_entry.S"));

/// Trap routines
#[unsafe(no_mangle)]
unsafe extern "C" fn trap_handler(
    mepc: usize,
    mtval: usize,
    mcause: usize,
    hart: usize,
    mstatus: usize,
    trap_frame: &mut TrapFrame,
) -> usize {
    let return_pc = mepc;
    // kprint_fmt!("trap frame: {:?}\n", trap_frame);
    // mcause strut -> u32 -> 31 bit = interrupt or exception.
    // if 31 bit is 1 -> interrupt, else 31 bit is 0 -> exception.
    // The remaining 30..0 bits is the interrupt or exception cause.
    // 31 bit[(interrupt or exception)] 30..0 bits[interrupt or exception cause]
    // Move all bits from mcause to 31 bits to the right to keep only the last bit
    // Last bit == interrupt
    let interrupt = mcause >> 31;
    // Bit mask to keep all bits except the last bit
    let cause = mcause & 0x7FFFFFFF;
    // Sanity check on mepc to panic if cannot mret
    if mepc == 0 || mepc == 0xFFFFFFFF {
        panic!("mepc value is wrong, cannot mret")
    }
    match interrupt {
        0 => exception_handler(cause, hart, mtval),
        1 => interrupt_handler(cause, hart),
        _ => panic!(
            "Reach unreachable point, last mcause bit has an incorrect value that the kernel cannot handle"
        ),
    }
    return_pc
}

fn exception_handler(mcause: usize, hart: usize, mtval: usize) {
    match mcause {
        1 => panic!("Instruction access fault: CPU#{}", hart),
        2 => panic!("Illegal instruction: CPU#{}", hart),
        5 => panic!(
            "Load access fault: CPU#{}\t instruction fault address: {:#x}",
            hart, mtval
        ),
        _ => panic!("Mcause exception raised: {}", mcause),
    }
}

fn interrupt_handler(mcause: usize, hart: usize) {
    match mcause {
        7 => timer_interrupt(hart),
        _ => panic!("Unhandled async trap CPU#{} -> {}\n", hart, mcause),
    }
}

fn timer_interrupt(hart: usize) {
    if hart == 0 {
        increment_tick();
    }
    set_ktime_ms(TICK_DURATION);
}
