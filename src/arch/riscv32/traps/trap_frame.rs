use core::{mem, ptr::null_mut};

#[repr(C)]
#[derive(Clone, Copy)]
// Trap frame structure, used to store all global registers, give a stack for the trap handling
// to avoid using the kernel stack.
pub struct TrapFrame {
    // Array to save all GP registers
    pub gp_regs: [u32; 32], // x0..x31  - integer registers
    // Supervisor Address Translation and Protection Register (satp register only exist when supervisor mode is enabled)
    pub satp: u32, // Offset in struct 128
    // Current hart id
    pub hartid: u32, // offset in struct 132
    // Mutable ptr to a bytes buffer to save trap stack
    pub trap_stack: *mut u8, // offset in struct 136
}

impl TrapFrame {
    // Initialized TrapFrame with field set to 0
    pub const fn zero() -> Self {
        TrapFrame {
            gp_regs: [0; 32],
            satp: 0,
            hartid: 0,
            trap_stack: null_mut(),
        }
    }
}

// Static buffer used as a stack for trap handling
static mut TRAP_STACK_BUFF: [u8; 1024] = [0u8; 1024];

// Init TrapFrame with 0 in mem
pub static mut KERNEL_TRAP_FRAME: TrapFrame = unsafe { mem::zeroed() };

/// Initialize trap frame with TRAP_STACK_BUFF static as TrapFrame.trap_stack
pub fn init_trap_frame() {
    // Static mut safe because it's only used in kernel boot
    #[allow(static_mut_refs)]
    unsafe {
        KERNEL_TRAP_FRAME.trap_stack = TRAP_STACK_BUFF.as_mut_ptr()
    }
}
