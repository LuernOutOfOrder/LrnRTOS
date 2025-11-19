use core::arch::asm;

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

pub fn enable_mie_mtie() {
    const MTIE: u32 = 1 << 7;
    unsafe { asm!("csrrs zero, mie, {}", in(reg) MTIE) };
}

pub fn enable_mie_msie() {
    const MSIE: u32 = 1 << 3;
    unsafe { asm!("csrrs zero, mie, {}", in(reg) MSIE) };
}

pub fn enable_mstatus_mie() {
    const MIE: u32 = 1 << 3;
    unsafe { asm!("csrrs zero, mstatus, {}", in(reg) MIE) };
}
