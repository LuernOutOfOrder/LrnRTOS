pub struct TaskContext {
    gpr: [u32; 32],
    address_space: [u32; 2],
    pc: u32,
    sp: u32,
    flags: [u8; 3],
    instruction_register: u8,
}
