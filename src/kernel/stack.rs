use core::mem;

pub struct KernelStack {
    pub top: usize,
    pub bottom: usize,
}

// Kernel image location
// From linker script symbol
unsafe extern "C" {
    pub static __kernel_start: u8;
    pub static __kernel_end: u8;
}

impl KernelStack {
    pub const fn init() -> Self {
        unsafe { mem::zeroed() }
    }
}
