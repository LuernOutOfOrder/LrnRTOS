use crate::primitives::stack::AlignedStack;

#[repr(C)]
pub struct SchedulerCtx {
    // General purpose registers
    gpr: [u32; 32], // Offset 0
    // Scheduler entry point
    ra: u32, // Offset 128
    // Stack ptr, point to a buffer
    sp: *mut u8, // Offset 132
}

pub static mut SCHEDULER_STACK: AlignedStack<4098> = AlignedStack::new();
pub static mut SCHEDULER_CTX: SchedulerCtx = unsafe { core::mem::zeroed() };

impl SchedulerCtx {
    fn init(func: fn()) -> Self {
        SchedulerCtx {
            gpr: [0u32; 32],
            ra: func as usize as u32,
            #[allow(static_mut_refs)]
            sp: unsafe { SCHEDULER_STACK.buf.as_mut_ptr().wrapping_add(4098) },
        }
    }
}

pub fn init_sched_ctx(sched_fn: fn()) {
    unsafe { SCHEDULER_CTX = SchedulerCtx::init(sched_fn) }
}

unsafe extern "C" {
    // Switch to the scheduler context
    pub fn sched_ctx_restore(context: *mut SchedulerCtx) -> !;
    // Save the scheduler context
    pub fn sched_ctx_save(context: usize);
}
