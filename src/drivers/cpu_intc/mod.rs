use core::cell::UnsafeCell;

use riscv_cpu_intc::RiscVCpuIntc;

use crate::config::CPU_INTC_MAX_SIZE;

pub mod riscv_cpu_intc;

pub trait CpuIntc {}

pub struct CpuIntcSubSystem {
    cpu_intc_pool: UnsafeCell<[Option<*mut dyn CpuIntc>; CPU_INTC_MAX_SIZE]>,
}

unsafe impl Sync for CpuIntcSubSystem {}

impl CpuIntcSubSystem {
    pub const fn init() -> Self {
        CpuIntcSubSystem {
            cpu_intc_pool: UnsafeCell::new([const { None }; CPU_INTC_MAX_SIZE]),
        }
    }

    /// Add a new driver for cpu interrupt-controller in the pool sub-system.
    ///
    /// Params:
    /// &self: the sub-system structure.
    /// cpu_intc: static structure of a driver implementing the CpuIntc trait.
    /// index: used to represente the cpu interrupt-controller core id, also used as an index in
    /// the sub-system pool. Because there's only one cpu interrupt-controller per cpu core, no
    /// overlap possible.
    pub fn add_cpu_intc(&self, cpu_intc: &'static mut dyn CpuIntc, index: usize) {
        let size = self.get_cpu_intc_array_size();
        if size == CPU_INTC_MAX_SIZE {
            panic!(
                "Cpu interrupt-controller sub-system pool possible overflow. Consider increase size in config file."
            )
        }
        unsafe {
            (&mut *self.cpu_intc_pool.get())[index] = Some(cpu_intc as *mut dyn CpuIntc);
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

    pub fn get_cpu_intc(&self, index: usize) -> Option<*mut dyn CpuIntc> {
        let cpu_intc_ptr = unsafe { (&*self.cpu_intc_pool.get())[index] };
        if let Some(ptr) = cpu_intc_ptr {
            // ptr was created from a &'static mut dyn CpuIntc in add_cpu_intc,
            // converting the raw pointer back to a &'static mut is safe.
            unsafe { Some(&mut *ptr) }
        } else {
            None
        }
    }
}

// Init static cpu interrupt-controller sub-system
pub static CPU_INTC_SUBSYSTEM: CpuIntcSubSystem = CpuIntcSubSystem::init();

/// Initialize the cpu interrupt-controller sub-system with all drivers available.
/// Call all cpu_intc driver init function, if the fn find a compatible node in the fdt, continue
/// the init and auto register itself in the sub-system. Else, if the init function doesn't find a
/// compatible node, it return to give the next driver init function the turn.
/// Panic if after all drivers init the sub-system pool is empty.
pub fn init_cpu_intc_subsystem() {
    RiscVCpuIntc::init();
    let size = CPU_INTC_SUBSYSTEM.get_cpu_intc_array_size();
    if size == 0 {
        panic!("Error while initializing Cpu interrupt-controller sub-system, pool is empty.");
    }
}
