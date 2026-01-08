use crate::tests::TestCase;

use super::{CpuIntcDriver, CpuIntcHw, CpuIntcSubSystem, riscv_cpu_intc::RiscVCpuIntc};

pub fn test_cpu_intc_subsystem_impl() {
    let cpu_intc_subsystem = CpuIntcSubSystem::init();
    if cpu_intc_subsystem.get_cpu_intc_array_size() != 0 {
        panic!("CPU interrupt-controller sub-system should be initialized empty.")
    }
    // Add CPU intc to sub-system
    let cpu_intc_pool: RiscVCpuIntc = RiscVCpuIntc { hart_id: 0 };
    let cpu_intc: CpuIntcHw = CpuIntcHw {
        driver: CpuIntcDriver::RiscVCpuIntc(cpu_intc_pool),
    };
    cpu_intc_subsystem.add_cpu_intc(cpu_intc, cpu_intc.get_cpu_intc_core_id() as usize);
    if cpu_intc_subsystem.get_cpu_intc_array_size() != 1 {
        panic!("CPU interrupt-controller sub-system should contain 1 CPU interrupt-controller.");
    }
    // Add second CPU intc to sub-system
    let cpu_intc_pool: RiscVCpuIntc = RiscVCpuIntc { hart_id: 1 };
    let cpu_intc: CpuIntcHw = CpuIntcHw {
        driver: CpuIntcDriver::RiscVCpuIntc(cpu_intc_pool),
    };
    cpu_intc_subsystem.add_cpu_intc(cpu_intc, cpu_intc.get_cpu_intc_core_id() as usize);

    // Check getting CPU intc
    // Unwrap because of the test env
    let get_cpu_intc = cpu_intc_subsystem.get_cpu_intc(0).unwrap();
    if get_cpu_intc.get_cpu_intc_core_id() != 0 {
        panic!("CPU interrupt-controller sub-system return the wrong CPU interrupt-controller");
    }
}

pub static CPU_INTC_SUBSYSTEM_TEST_SUITE: &[TestCase] = &[TestCase {
    name: "CPU interrupt-controller sub-system basic implementation",
    func: test_cpu_intc_subsystem_impl,
}];
