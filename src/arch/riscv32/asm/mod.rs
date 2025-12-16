use core::arch::global_asm;

global_asm!(include_str!("kernel_sp.S"));

unsafe extern "C" {
    pub fn kernel_sp();
}
