use core::arch::asm;

use super::handler::{TrapFrame, KERNEL_TRAP_FRAME};

/// Enable interrupt
///
/// # Safety
///
/// .
pub unsafe fn enable_interrupt() {
    unsafe { asm!("csrsi sstatus, 1 << 1") }
}

/// .
///
/// # Safety
///
/// .
pub unsafe fn disable_interrupt() {
    unsafe { asm!("csrci sstatus, 1 << 1") }
}

/// .
///
/// # Safety
///
/// .
pub unsafe fn halt() {
    unsafe { asm!("wfi", options(nomem, nostack)) }
}

/// Set interrupts and halt
/// This will atomically wait for the next interrupt
/// Performing enable followed by halt is not guaranteed to be atomic, use this instead!
/// # Safety
pub unsafe fn enable_and_halt() {
    unsafe { asm!("wfi", "csrsi sstatus, 1 << 1", "nop") }
}

// Machine Interrupt Enable Register

pub fn enable_mie_mtie() {
    const MTIE: u32 = 1 << 7;
    unsafe { asm!("csrrs zero, mie, {}", in(reg) MTIE) };
}

pub fn disable_mie_mtie() {
    const MTIE: u32 = 1 << 7;
    unsafe { asm!("csrrc zero, mie, {}", in(reg) MTIE) };
}

pub fn enable_mie_msie() {
    const MSIE: u32 = 1 << 3;
    unsafe { asm!("csrrs zero, mie, {}", in(reg) MSIE) };
}

// Machine Status

pub fn enable_mstatus_mie() {
    const MIE: u32 = 1 << 3;
    unsafe { asm!("csrrs zero, mstatus, {}", in(reg) MIE) };
}

// Machine Trap-Vector

pub fn mtvec_switch_to_vectored_mode() {
    const MODE: u32 = 1 << 0;
    unsafe { asm!("csrrs zero, mtvec, {}", in(reg) MODE) };
}

pub fn mtvec_switch_to_direct_mode() {
    const MODE: u32 = 1 << 0;
    unsafe { asm!("csrrc zero, mtvec, {}", in(reg) MODE) };
}

unsafe extern "C" {
    pub fn trap_entry();
}

pub fn mtvec_set_trap_entry() {
    let handler_ptr: unsafe extern "C" fn() = trap_entry;
    unsafe { asm!("csrw mtvec, {}", in(reg) handler_ptr) }
}

pub fn mscratch_set_trap_frame() {
    let ptr = (&mut unsafe { KERNEL_TRAP_FRAME }[0]
		                     as *mut TrapFrame)
		                    as usize;
    unsafe { asm!("csrw mscratch, {}", in(reg) ptr) }
}
