use core::{arch::global_asm, ptr::null_mut};

use crate::{ktime::set_ktime_ms, print};

use super::interrupt::trap_entry;

// Include gnu_macro asm file in compilation
global_asm!(include_str!("gnu_macro.S"));
// Include trap_entry asm file for trap entry fn in compilation
global_asm!(include_str!("trap_entry.S"));

#[repr(C)]
#[derive(Clone, Copy)]
pub struct TrapFrame {
    pub gp_regs: [u32; 32], // x0..x31  - integer registers
    pub fp_regs: [u64; 32], // f0..f31  - floating-point registers (64-bit wide)
    pub satp: u32,
    pub trap_stack: *mut u8, // offset in struct 388
    pub hartid: u32,
}

impl TrapFrame {
    pub const fn zero() -> Self {
        TrapFrame {
            gp_regs: [0; 32],
            fp_regs: [0; 32],
            satp: 0,
            trap_stack: null_mut(),
            hartid: 0,
        }
    }
}

pub static mut KERNEL_TRAP_FRAME: [TrapFrame; 8] = [TrapFrame::zero(); 8];

/// Trap routines
#[unsafe(no_mangle)]
unsafe extern "C" fn trap_handler(
    mepc: u32,
    tval: usize,
    mcause: u32,
    hart: usize,
    status: usize,
    frame: &mut TrapFrame,
) -> u32 {
    print!("DEBUG\n");
    // mcause strut -> u32 -> 31 bit = interrupt or exception.
    // if 31 bit is 1 -> interrupt, else 31 bit is 0 -> exception.
    // The remaining 30..0 bits is the interrupt or exception cause.
    // 31 bit[(interrupt or exception)] 30..0 bits[interrupt or exception cause]
    // Move all bits from mcause to 31 bits to the right to keep only the last bit
    // Last bit == interrupt
    let interrupt = mcause >> 31;
    // Bit mask to keep all bits except the last bit
    let cause = mcause & 0x7FFFFFFF;
    let updated_pc = mepc;
    let trap_handler_addr = trap_entry as usize;
    match mepc {
        0 => panic!("mepc is 0, wrong wrong wrong"),
        0xFFFFFFFF => panic!("mepc is like BIG, so wrong wrong wrong"),
        _ => (),
    }
    if mepc as usize == trap_handler_addr {
        panic!("mecp shouldn't point to trap_entry addr")
    }
    match interrupt {
        0 => exception_handler(cause),
        1 => interrupt_handler(cause, hart),
        _ => panic!(
            "Reach unreachable point, last mcause bit has an incorrect value that the kernel cannot handle"
        ),
    }
    updated_pc
}

fn exception_handler(mcause: u32) {
    match mcause {
        1 => panic!("Instruction access fault"),
        _ => panic!("Mcause exception raised"),
    }
}

fn interrupt_handler(mcause: u32, hart: usize) {
    match mcause {
        7 => timer_interrupt(),
        _ => panic!("Unhandled async trap CPU#{} -> {}\n", hart, mcause),
    }
}

fn timer_interrupt() {
    print!("Timer interrupt\n");
    set_ktime_ms(10_000_000);
}
