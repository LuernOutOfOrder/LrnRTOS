use crate::{
    config::CPU_INTC_MAX_SIZE,
    drivers::cpu_intc::{riscv_cpu_intc::RiscVCpuIntc, CpuIntcDriver, CpuIntcHw, CpuIntcSubSystem},
    tests::{TestCase, TEST_MANAGER},
};

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
    let cpu_intc_pool1: RiscVCpuIntc = RiscVCpuIntc { hart_id: 1 };
    let cpu_intc1: CpuIntcHw = CpuIntcHw {
        driver: CpuIntcDriver::RiscVCpuIntc(cpu_intc_pool1),
    };
    cpu_intc_subsystem.add_cpu_intc(cpu_intc1, cpu_intc1.get_cpu_intc_core_id() as usize);

    // Check getting CPU intc
    // Unwrap because of the test env
    let get_cpu_intc = cpu_intc_subsystem.get_cpu_intc(0).unwrap();
    if get_cpu_intc.get_cpu_intc_core_id() != 0 {
        panic!("CPU interrupt-controller sub-system return the wrong CPU interrupt-controller");
    }
}

pub fn test_cpu_intc_subsystem_same_device() {
    let cpu_intc_subsystem = CpuIntcSubSystem::init();
    // Add CPU intc to sub-system
    let cpu_intc_pool: RiscVCpuIntc = RiscVCpuIntc { hart_id: 0 };
    let cpu_intc: CpuIntcHw = CpuIntcHw {
        driver: CpuIntcDriver::RiscVCpuIntc(cpu_intc_pool),
    };
    // Add second CPU intc to sub-system
    let cpu_intc_pool1: RiscVCpuIntc = RiscVCpuIntc { hart_id: 0 };
    let cpu_intc1: CpuIntcHw = CpuIntcHw {
        driver: CpuIntcDriver::RiscVCpuIntc(cpu_intc_pool1),
    };
    cpu_intc_subsystem.add_cpu_intc(cpu_intc, cpu_intc.get_cpu_intc_core_id() as usize);
    // This one should trigger a warning for duplication
    cpu_intc_subsystem.add_cpu_intc(cpu_intc1, cpu_intc1.get_cpu_intc_core_id() as usize);
    // Check CPU intc subsystem size
    if cpu_intc_subsystem.get_cpu_intc_array_size() != 1 {
        panic!(
            "CPU interrupt-controller sub-system should contain only 1 CPU interrupt-controller."
        );
    }
    // Check getting CPU intc
    let get_cpu_intc = cpu_intc_subsystem.get_cpu_intc(0).unwrap();
    if get_cpu_intc.get_cpu_intc_core_id() != 0 {
        panic!("CPU interrupt-controller sub-system return the wrong CPU interrupt-controller");
    }
}

pub fn test_cpu_intc_subsystem_overflow() {
    let cpu_intc_subsystem = CpuIntcSubSystem::init();
    // Add CPU intc to sub-system
    let cpu_intc_pool: RiscVCpuIntc = RiscVCpuIntc { hart_id: 0 };
    let cpu_intc: CpuIntcHw = CpuIntcHw {
        driver: CpuIntcDriver::RiscVCpuIntc(cpu_intc_pool),
    };
    // Add second CPU intc to sub-system
    let cpu_intc_pool1: RiscVCpuIntc = RiscVCpuIntc { hart_id: 1 };
    let cpu_intc1: CpuIntcHw = CpuIntcHw {
        driver: CpuIntcDriver::RiscVCpuIntc(cpu_intc_pool1),
    };
    // Add second CPU intc to sub-system
    let cpu_intc_pool2: RiscVCpuIntc = RiscVCpuIntc { hart_id: 2 };
    let cpu_intc2: CpuIntcHw = CpuIntcHw {
        driver: CpuIntcDriver::RiscVCpuIntc(cpu_intc_pool2),
    };
    // Add all CPU intc to subsystem
    cpu_intc_subsystem.add_cpu_intc(cpu_intc, cpu_intc.get_cpu_intc_core_id() as usize);
    cpu_intc_subsystem.add_cpu_intc(cpu_intc1, cpu_intc1.get_cpu_intc_core_id() as usize);
    cpu_intc_subsystem.add_cpu_intc(cpu_intc2, cpu_intc2.get_cpu_intc_core_id() as usize);
    // Check CPU intc subsystem size
    if cpu_intc_subsystem.get_cpu_intc_array_size() > CPU_INTC_MAX_SIZE {
        panic!("CPU interrupt-controller sub-system should not exceed max size.");
    }

    // Check getting CPU intc
    let get_cpu_intc = cpu_intc_subsystem.get_cpu_intc(0).unwrap();
    if get_cpu_intc.get_cpu_intc_core_id() != 0 {
        panic!("CPU interrupt-controller sub-system return the wrong CPU interrupt-controller");
    }
}

pub fn cpu_intc_subsystem_test_suite() {
    let cpu_intc_subsystem_test_suite: &[TestCase] = &[
        TestCase {
            name: "CPU interrupt-controller sub-system basic implementation",
            func: test_cpu_intc_subsystem_impl,
        },
        TestCase {
            name: "CPU interrupt-controller sub-system add same CPU interrupt-controller",
            func: test_cpu_intc_subsystem_same_device,
        },
        TestCase {
            name: "CPU interrupt-controller sub-system handling overflow",
            func: test_cpu_intc_subsystem_overflow,
        },
    ];
    #[allow(static_mut_refs)]
    unsafe {
        TEST_MANAGER.add_suite(cpu_intc_subsystem_test_suite)
    };
}
