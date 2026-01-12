use core::arch::asm;

use super::trap_frame::KERNEL_TRAP_FRAME;

/// Enable supervisor interrupt
///
/// # Safety
///
/// .
pub unsafe fn enable_interrupt() {
    unsafe { asm!("csrsi sstatus, 1 << 1") }
}

/// Disable supervisor interrupt
///
/// # Safety
///
/// .
pub unsafe fn disable_interrupt() {
    unsafe { asm!("csrci sstatus, 1 << 1") }
}

/// Wait For Interrupt instruction
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

// Machine Interrupt Enable CSR

pub fn enable_mie_mtie() {
    // Set the 7 bit to 1
    const MTIE: u32 = 1 << 7;
    unsafe { asm!("csrrs zero, mie, {}", in(reg) MTIE) };
}

pub fn disable_mie_mtie() {
    // Clear the seven bit to 0
    const MTIE: u32 = 1 << 7;
    unsafe { asm!("csrrc zero, mie, {}", in(reg) MTIE) };
}

pub fn enable_mie_msie() {
    // Set the 3 bit to 1
    const MSIE: u32 = 1 << 3;
    unsafe { asm!("csrrs zero, mie, {}", in(reg) MSIE) };
}

pub fn disable_mie_msie() {
    // Clear the 3 bit to 0
    const MSIE: u32 = 1 << 3;
    unsafe { asm!("csrrc zero, mie, {}", in(reg) MSIE) };
}

// Machine Status CSR

pub fn enable_mstatus_mie() {
    // Set the 3 bit to 1
    const MIE: u32 = 1 << 3;
    unsafe { asm!("csrrs zero, mstatus, {}", in(reg) MIE) };
}

pub fn disable_mstatus_mie() {
    // Set the 3 bit to 1
    const MIE: u32 = 1 << 3;
    unsafe { asm!("csrrc zero, mstatus, {}", in(reg) MIE) };
}

// Machine Trap-Vector CSR

pub fn mtvec_switch_to_vectored_mode() {
    // Set the first bit to 1
    const MODE: u32 = 1 << 0;
    unsafe { asm!("csrrs zero, mtvec, {}", in(reg) MODE) };
}

pub fn mtvec_switch_to_direct_mode() {
    // Clear the first bit to 0
    const MODE: u32 = 1 << 0;
    unsafe { asm!("csrrc zero, mtvec, {}", in(reg) MODE) };
}

pub fn mtvec_read_mode() -> u32 {
    let value: u32; 
    unsafe {
        asm!("csrr {}, mtvec", out(reg) value);
    } 
    // Shift to keep only the bit 1:0
    value & 0b11
}

// Extern symbol for trap_entry function, this function is wrote in asm so it need to be extern
unsafe extern "C" {
    pub fn trap_entry();
}

// Set the trap_entry address in mtvec
pub fn mtvec_set_trap_entry() {
    // trap_entry address
    let handler_ptr: unsafe extern "C" fn() = trap_entry;
    unsafe { asm!("csrw mtvec, {}", in(reg) handler_ptr) }
}

// Mscratch CSR

pub fn mscratch_set_trap_frame() {
    #[allow(static_mut_refs)]
    // Ptr to KERNEL_TRAP_FRAME static
    let ptr = unsafe { &mut KERNEL_TRAP_FRAME } as *mut _ as usize;
    unsafe { asm!("csrw mscratch, {}", in(reg) ptr) }
}

pub fn mscratch_read() -> u32 {
    let value: u32;
    unsafe {
        core::arch::asm!(
            "csrr {0}, mscratch",
            out(reg) value
        );
    }
    value
}
