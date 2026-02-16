#[repr(C)]
#[derive(Copy, Clone)]
pub struct KArchCtx {
    pub gpr: [u32; 32], // Offset 0
    pub pc: u32,        // Offset 136
    pub sp: u32,        // Offset 140
    pub ra: u32,        // Offset 144
    pub mstatus: u32,   // Offset 148
}

impl KArchCtx {
    pub const fn init() -> Self {
        KArchCtx {
            gpr: [0u32; 32],
            pc: 0,
            sp: 0,
            ra: 0,
            // Keep the mstatus.mie enable
            mstatus: 8,
        }
    }
}
