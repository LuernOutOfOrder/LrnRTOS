use interrupt::{
    disable_mie_msie, disable_mie_mtie, disable_mstatus_mie, enable_mie_msie, enable_mie_mtie,
    enable_mstatus_mie, mscratch_set_trap_frame, mtvec_set_trap_entry, mtvec_switch_to_direct_mode,
};

pub mod handler;
pub mod interrupt;
pub mod misc;

pub fn enable_interrupts() {
    // Enable direct mode in mtvec
    mtvec_switch_to_direct_mode();
    mtvec_set_trap_entry();
    // Set trap frame in mscratch csr
    mscratch_set_trap_frame();
    // Enable timer interrupt
    enable_mie_mtie();
    // Enable softsare interrupt
    enable_mie_msie();
    // Enable interrupt handling in exception handler
    enable_mstatus_mie();
}

pub fn disable_interrupts() {
    disable_mie_mtie();
    disable_mie_msie();
    disable_mstatus_mie();
}
