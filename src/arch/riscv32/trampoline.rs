use core::arch::naked_asm;

#[unsafe(naked)]
pub fn save_ra() -> usize {
    unsafe { naked_asm!("mv a0, ra", "ret") };
}

#[unsafe(naked)]
pub fn save_sp() -> usize {
    unsafe { naked_asm!("mv a0, sp", "ret") };
}
