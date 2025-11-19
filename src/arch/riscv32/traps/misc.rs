#[inline(always)]
pub fn read_mstatus() -> u32 {
    let value: u32;
    unsafe {
        core::arch::asm!(
            "csrr {0}, mstatus",
            out(reg) value
        );
    }
    value
}

#[inline(always)]
pub fn read_mie() -> u32 {
    let value: u32;
    unsafe {
        core::arch::asm!(
            "csrr {0}, mie",
            out(reg) value
        );
    }
    value
}

pub fn read_mtvec() -> u32 {
    let value: u32;
    unsafe {
        core::arch::asm!(
            "csrr {0}, mtvec",
            out(reg) value
        );
    }
    value
}
pub fn mstatus_mie_is_set() -> bool {
    let mstatus = read_mstatus();
    (mstatus & (1 << 3)) != 0
}

pub fn mstatus_mpie_is_set() -> bool {
    let mstatus = read_mstatus();
    (mstatus & (1 << 7)) != 0
}

pub fn mie_mtie_is_set() -> bool {
    let mie = read_mie();
    (mie & (1 << 7)) != 0
}

pub fn mtvec_mode() -> u32 {
    let mtvec = read_mtvec();
    mtvec & (1 << 0)
}
