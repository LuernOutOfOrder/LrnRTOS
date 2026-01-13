/*
File info: CPU interrupt-controller sub-system.

Test coverage: Basic test and some edge case tested.

Tested:
- All basic method from implementation.
- Adding the same CPU interrupt-controller.
- Overflow in the sub-system pool.

Not tested:
- ...

Reasons:
- ...

Tests files:
- 'src/tests/drivers/cpu_intc/subsystem.rs'
*/

use core::cell::UnsafeCell;

use riscv_cpu_intc::RiscVCpuIntc;

use crate::{config::CPU_INTC_MAX_SIZE, log, logs::LogLevel};

pub mod riscv_cpu_intc;

// Trait to implement in all cpu interrupt-controller driver
pub trait CpuIntc {}

#[derive(Copy, Clone)]
// Unions enum for CpuIntcDriver struct
// avoid using &'static mut dyn CpuIntc
pub enum CpuIntcDriver {
    #[allow(unused)]
    RiscVCpuIntc(RiscVCpuIntc),
}

#[derive(Copy, Clone)]
pub struct CpuIntcHw {
    #[allow(unused)]
    pub driver: CpuIntcDriver,
}

impl CpuIntcHw {
    pub fn get_cpu_intc_core_id(&self) -> u32 {
        match self.driver {
            CpuIntcDriver::RiscVCpuIntc(riscv_cpu_intc) => riscv_cpu_intc.hart_id,
        }
    }
}

// Structure handling the cpu interrupt-controller initialized drivers
pub struct CpuIntcSubSystem {
    cpu_intc_pool: UnsafeCell<[Option<CpuIntcHw>; CPU_INTC_MAX_SIZE]>,
}

unsafe impl Sync for CpuIntcSubSystem {}

impl CpuIntcSubSystem {
    pub const fn init() -> Self {
        CpuIntcSubSystem {
            cpu_intc_pool: UnsafeCell::new([const { None }; CPU_INTC_MAX_SIZE]),
        }
    }

    /// Add a new driver for CPU interrupt-controller in the pool sub-system.
    ///
    /// Params:
    /// &self: the sub-system structure.
    /// cpu_intc: structure of a CPU interrupt-controller "driver".
    /// index: used to represent the CPU interrupt-controller core id, also used as an index in
    /// the sub-system pool. Because there's only one CPU interrupt-controller per CPU core, no
    /// overlap possible.
    pub fn add_cpu_intc(&self, new_cpu_intc: CpuIntcHw, index: usize) {
        let size = self.get_cpu_intc_array_size();
        if size == CPU_INTC_MAX_SIZE {
            log!(
                LogLevel::Warn,
                "CPU interrupt-controller sub-system: sub-system is full, ignoring registration request"
            );
            return;
        }
        if self.get_cpu_intc(index).is_some() {
            log!(
                LogLevel::Warn,
                "CPU interrupt-controller sub-system: duplicate device detected, ignoring registration request"
            );
        } else {
            unsafe {
                (&mut *self.cpu_intc_pool.get())[index] = Some(new_cpu_intc);
            }
        }
    }

    pub fn get_cpu_intc_array_size(&self) -> usize {
        let mut size: usize = 0;
        for i in 0..CPU_INTC_MAX_SIZE {
            let present = unsafe { (&*self.cpu_intc_pool.get())[i].is_some() };
            if present {
                size += 1;
            }
        }
        size
    }

    pub fn get_cpu_intc(&self, index: usize) -> Option<&CpuIntcHw> {
        let cpu_intc = unsafe { &(*self.cpu_intc_pool.get())[index] };
        if let Some(cpu_intc_driver) = cpu_intc {
            Some(cpu_intc_driver)
        } else {
            None
        }
    }
}

// Init static CPU interrupt-controller sub-system
pub static CPU_INTC_SUBSYSTEM: CpuIntcSubSystem = CpuIntcSubSystem::init();

pub fn init_cpu_intc_subsystem() {
    RiscVCpuIntc::init();
    let size = CPU_INTC_SUBSYSTEM.get_cpu_intc_array_size();
    if size == 0 {
        panic!("Error while initializing CPU interrupt-controller sub-system, pool is empty.");
    }
}
