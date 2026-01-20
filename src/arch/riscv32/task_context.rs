pub struct TaskContext {
    pub gpr: [u32; 32],
    pub address_space: [u32; 2],
    pub pc: u32,
    pub sp: u32,
    pub flags: [u8; 3],
    pub instruction_register: u8,
}

impl TaskContext {
    pub const fn init(size: [usize; 2]) -> Self {
        TaskContext {
            gpr: [0u32; 32],
            address_space: [size[0] as u32, size[1] as u32],
            pc: 0,
            sp: 0,
            flags: [0u8; 3],
            instruction_register: 0,
        }
    }
}
