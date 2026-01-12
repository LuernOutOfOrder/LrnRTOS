/*
File info: Enable all needed interruptions in RISC-V 32 bits CPU.

Test coverage: All enable_interrupts fn.

Tested:
- All the functions used in enable_interrupts fn. 
  
Not tested:
- The disable_interrupts fn
  
Reasons:
- Not even used yet so I'm not bothering myself to write a test on that.

Tests files:
- 'src/tests/arch/riscv32/traps/interrupt.rs'
*/

use interrupt::{
    disable_mie_msie, disable_mie_mtie, disable_mstatus_mie, enable_mie_msie, enable_mie_mtie,
    enable_mstatus_mie, mscratch_set_trap_frame, mtvec_set_trap_entry, mtvec_switch_to_direct_mode,
};

pub mod handler;
pub mod interrupt;
pub mod misc;
pub mod trap_frame;

/// Enable all needed interruptions
pub fn enable_interrupts() {
    // Enable direct mode in mtvec
    mtvec_switch_to_direct_mode();
    // Set the trap_entry fn in mtvec direct mode
    mtvec_set_trap_entry();
    // Set trap frame in mscratch CSR
    mscratch_set_trap_frame();
    // Enable timer interrupt
    enable_mie_mtie();
    // Enable software interrupt
    enable_mie_msie();
    // Enable interrupt handling in exception handler
    enable_mstatus_mie();
}

/// Disable all interruptions
pub fn disable_interrupts() {
    disable_mie_mtie();
    disable_mie_msie();
    disable_mstatus_mie();
}
