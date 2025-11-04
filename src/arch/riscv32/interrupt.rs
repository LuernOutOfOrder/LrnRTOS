use core::arch::asm;

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
