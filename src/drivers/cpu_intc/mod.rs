use core::cell::UnsafeCell;

use riscv_cpu_intc::RiscVCpuIntc;

use crate::config::CPU_INTC_MAX_SIZE;

pub mod riscv_cpu_intc;

pub trait CpuIntc {}

pub struct CpuIntcSubSystem {
    cpu_intc_pool: UnsafeCell<[Option<&'static mut dyn CpuIntc>; CPU_INTC_MAX_SIZE]>,
}

unsafe impl Sync for CpuIntcSubSystem {}

impl CpuIntcSubSystem {
    pub const fn init() -> Self {
        CpuIntcSubSystem {
            cpu_intc_pool: UnsafeCell::new([const { None }; CPU_INTC_MAX_SIZE]),
        }
    }

    pub fn add_cpu_intc(&self, cpu_intc: &'static mut dyn CpuIntc) {
        let size = self.get_cpu_intc_array_size();
        if size == CPU_INTC_MAX_SIZE {
            panic!(
                "Cpu interrupt-controller sub-system pool possible overflow. Consider increase size in config file."
            )
        }
        let mut cpu_intc_subsystem_array_index: usize = 0;
        for i in 0..CPU_INTC_MAX_SIZE {
            let cpu_intc = unsafe { (&*self.cpu_intc_pool.get())[i].as_ref() };
            if cpu_intc.is_none() {
                cpu_intc_subsystem_array_index = i;
                break;
            }
        }
        unsafe {
            (&mut *self.cpu_intc_pool.get())[cpu_intc_subsystem_array_index] = Some(cpu_intc);
        }
    }

    pub fn get_cpu_intc_array_size(&self) -> usize {
        let mut size: usize = 0;
        for i in 0..CPU_INTC_MAX_SIZE {
            let cpu_intc = unsafe { (&*self.cpu_intc_pool.get())[i].as_ref() };
            if cpu_intc.is_some() {
                size += 1;
            }
        }
        size
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
