use interrupt::{enable_mie_msie, enable_mie_mtie, enable_mstatus_mie, mtvec_set_trap_entry, mtvec_switch_to_direct_mode};

pub mod interrupt;
pub mod misc;
pub mod handler;

pub fn enable_interrupts() {
    // Enable timer interrupt
    enable_mie_mtie();
    // Enable softsare interrupt
    enable_mie_msie();
    // Enable direct mode in mtvec
    mtvec_switch_to_direct_mode();
    mtvec_set_trap_entry();
    // Enable interrupt handling in exception handler
    enable_mstatus_mie();
}
