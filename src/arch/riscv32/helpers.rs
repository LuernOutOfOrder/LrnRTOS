use crate::arch::asm;

pub fn current_cpu_core() -> usize {
    let id: usize = 0;
    unsafe { asm!("csrr {}, mhartid", out(reg) id) };
    id
}
