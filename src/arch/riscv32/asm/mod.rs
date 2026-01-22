use core::arch::global_asm;

global_asm!(include_str!("set_kernel_sp.S"));
// Include gnu_macro asm file in compilation
global_asm!(include_str!("gnu_macro.S"));

unsafe extern "C" {
    pub fn set_kernel_sp();
}
