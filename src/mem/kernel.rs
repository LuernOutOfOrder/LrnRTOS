pub struct KernelStack {
    pub top: usize,
    pub bottom: usize,
}

pub static mut KERNEL_STACK: KernelStack = KernelStack { top: 0, bottom: 0 };
