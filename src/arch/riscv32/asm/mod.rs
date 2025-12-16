use core::arch::global_asm;

global_asm!(include_str!("set_kernel_sp.S"));

unsafe extern "C" {
    pub fn set_kernel_sp();
}
