use crate::{primitives::stack::AlignedStack, scheduler::dispatch};

pub struct SchedulerCtx {
    // General purpose registers
    gpr: [u32; 32], // Offset 0
    // Scheduler entry point
    func: fn(), // Offset 128
    // Stack ptr, point to a buffer
    sp: *mut u8, // Offset 132
}

static mut SCHEDULER_STACK: AlignedStack<1024> = AlignedStack::new();
pub static mut SCHEDULER_CTX: SchedulerCtx = SchedulerCtx::init(dispatch);

impl SchedulerCtx {
    const fn init(func: fn()) -> Self {
        SchedulerCtx {
            gpr: [0u32; 32],
            func,
            #[allow(static_mut_refs)]
            sp: unsafe { SCHEDULER_STACK.buf.as_mut_ptr() },
        }
    }
}

unsafe extern "C" {
    // Switch to the scheduler context
    pub fn sched_ctx(context: usize);
}
