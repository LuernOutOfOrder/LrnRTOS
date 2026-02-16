use crate::{arch::kernel::KArchCtx, mem::mem_kernel_stack_info};

use super::stack::KernelStack;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct KContext {
    context: KArchCtx,
    pub flags: [u8; 3],
}

impl KContext {
    pub const fn init() -> Self {
        KContext {
            context: KArchCtx::init(),
            flags: [0u8; 3],
        }
    }
}

pub static mut KCONTEXT: KContext = KContext::init();

pub fn kcontext_init() {}
