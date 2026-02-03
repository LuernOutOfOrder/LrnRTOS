use super::asm;

pub fn update_kernel_sp(sp: usize) {
    unsafe { core::arch::asm!("mv a0, {}", in(reg) sp) };
    unsafe { asm::set_kernel_sp() };
}
