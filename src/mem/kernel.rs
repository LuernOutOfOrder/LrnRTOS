use core::mem;

pub struct KernelStack {
    pub top: usize,
    pub bottom: usize,
}

impl KernelStack {
    pub const fn init() -> Self {
        unsafe { mem::zeroed() }
    }
}

pub static mut KERNEL_STACK: KernelStack = KernelStack::init();
