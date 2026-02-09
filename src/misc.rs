use crate::{arch, config::CPU_CORE_NUMBER};

#[repr(C)]
pub struct RawTraitObject {
    pub data: *const (),
    pub vtable: *const (),
}

pub static mut CPUS_STATE: CpusState = CpusState::init();

#[repr(C)]
struct CpusState {
    // Flags for the CPU states, not used yet.
    cpu_state: [u8; CPU_CORE_NUMBER],
    // Flags for the CPU scheduler state.
    // bit 0: scheduler state, init or not.
    // bit 1: need reschedule or not.
    // bit 2:7: reschedule reason.
    scheduler_state: [u8; CPU_CORE_NUMBER],
}

impl CpusState {
    const fn init() -> Self {
        CpusState {
            cpu_state: [0u8; CPU_CORE_NUMBER],
            scheduler_state: [0u8; CPU_CORE_NUMBER],
        }
    }

    fn scheduler_set_reschedule_bit(&mut self, core: usize) {
        let mut state = self.scheduler_state[core];
        let mask = 1 << 1;
        // Set need reschedule bit.
        state = state | mask;
    }

    fn scheduler_clear_reschedule_bit(&mut self, core: usize) {
        let mut state = self.scheduler_state[core];
        let mask = 0 << 1;
        // Clear need reschedule bit.
        state = state | mask;
    }

    fn scheduler_read_reschedule_bit(&self, core: usize) -> bool {
        let mut state = self.scheduler_state[core];
        let mask = 1 << 1;
        // Get the bit 1
        state = state & mask;
    }
}

pub fn need_reschedule() {
    let current_core = arch::helpers::current_cpu_core();
    #[allow(static_mut_refs)]
    unsafe {
        CPUS_STATE.scheduler_set_reschedule_bit(current_core)
    };
}

pub fn clear_reschedule() {
    let current_core = arch::helpers::current_cpu_core();
    #[allow(static_mut_refs)]
    unsafe {
        CPUS_STATE.scheduler_clear_reschedule_bit(current_core)
    };
}

pub fn read_need_reschedule() {
    let current_core = arch::helpers::current_cpu_core();
    #[allow(static_mut_refs)]
    unsafe {
        CPUS_STATE.scheduler_read_reschedule_bit(current_core)
    };
}
