use crate::{
    config::CPU_INTC_MAX_SIZE,
    drivers::cpu_intc::{CpuIntcDriver, CpuIntcHw, CpuIntcSubSystem, riscv_cpu_intc::RiscVCpuIntc},
    tests::{TEST_MANAGER, TestBehavior, TestCase, TestSuite, TestSuiteBehavior},
};

pub fn test_cpu_intc_subsystem_impl() -> u8 {
    let cpu_intc_subsystem = CpuIntcSubSystem::init();
    if cpu_intc_subsystem.get_cpu_intc_array_size() != 0 {
        panic!("CPU interrupt-controller sub-system should be initialized empty.")
    }
    // Add CPU intc to sub-system
    let cpu_intc_pool: RiscVCpuIntc = RiscVCpuIntc { hart_id: 0 };
    let cpu_intc: CpuIntcHw = CpuIntcHw {
        driver: CpuIntcDriver::RiscVCpuIntc(cpu_intc_pool),
    };
    let cpu_core_id = &cpu_intc.get_cpu_intc_core_id();
    cpu_intc_subsystem.add_cpu_intc(cpu_intc, *cpu_core_id as usize);
    if cpu_intc_subsystem.get_cpu_intc_array_size() != 1 {
        panic!("CPU interrupt-controller sub-system should contain 1 CPU interrupt-controller.");
    }
    // Add second CPU intc to sub-system
    let cpu_intc_pool1: RiscVCpuIntc = RiscVCpuIntc { hart_id: 1 };
    let cpu_intc1: CpuIntcHw = CpuIntcHw {
        driver: CpuIntcDriver::RiscVCpuIntc(cpu_intc_pool1),
    };
    let cpu_core_id1 = &cpu_intc1.get_cpu_intc_core_id();
    cpu_intc_subsystem.add_cpu_intc(cpu_intc1, *cpu_core_id1 as usize);

    // Check getting CPU intc
    // Unwrap because of the test env
    let get_cpu_intc = cpu_intc_subsystem.get_cpu_intc(0).unwrap();
    if get_cpu_intc.get_cpu_intc_core_id() != 0 {
        panic!("CPU interrupt-controller sub-system return the wrong CPU interrupt-controller");
    }
    0
}

pub fn test_cpu_intc_subsystem_same_device() -> u8 {
    let cpu_intc_subsystem = CpuIntcSubSystem::init();
    // Add CPU intc to sub-system
    let cpu_intc_pool: RiscVCpuIntc = RiscVCpuIntc { hart_id: 0 };
    let cpu_intc: CpuIntcHw = CpuIntcHw {
        driver: CpuIntcDriver::RiscVCpuIntc(cpu_intc_pool),
    };
    let cpu_core_id = &cpu_intc.get_cpu_intc_core_id();
    // Add second CPU intc to sub-system
    let cpu_intc_pool1: RiscVCpuIntc = RiscVCpuIntc { hart_id: 0 };
    let cpu_intc1: CpuIntcHw = CpuIntcHw {
        driver: CpuIntcDriver::RiscVCpuIntc(cpu_intc_pool1),
    };
    let cpu_core_id1 = &cpu_intc1.get_cpu_intc_core_id();
    cpu_intc_subsystem.add_cpu_intc(cpu_intc, *cpu_core_id as usize);
    // This one should trigger a warning for duplication
    cpu_intc_subsystem.add_cpu_intc(cpu_intc1, *cpu_core_id1 as usize);
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
    0
}

pub fn test_cpu_intc_subsystem_overflow() -> u8 {
    let cpu_intc_subsystem = CpuIntcSubSystem::init();
    // Add CPU intc to sub-system
    let cpu_intc_pool: RiscVCpuIntc = RiscVCpuIntc { hart_id: 0 };
    let cpu_intc: CpuIntcHw = CpuIntcHw {
        driver: CpuIntcDriver::RiscVCpuIntc(cpu_intc_pool),
    };
    let cpu_core_id = &cpu_intc.get_cpu_intc_core_id();
    // Add second CPU intc to sub-system
    let cpu_intc_pool1: RiscVCpuIntc = RiscVCpuIntc { hart_id: 1 };
    let cpu_intc1: CpuIntcHw = CpuIntcHw {
        driver: CpuIntcDriver::RiscVCpuIntc(cpu_intc_pool1),
    };
    let cpu_core_id1 = &cpu_intc1.get_cpu_intc_core_id();
    // Add second CPU intc to sub-system
    let cpu_intc_pool2: RiscVCpuIntc = RiscVCpuIntc { hart_id: 2 };
    let cpu_intc2: CpuIntcHw = CpuIntcHw {
        driver: CpuIntcDriver::RiscVCpuIntc(cpu_intc_pool2),
    };
    let cpu_core_id2 = &cpu_intc2.get_cpu_intc_core_id();
    // Add all CPU intc to subsystem
    cpu_intc_subsystem.add_cpu_intc(cpu_intc, *cpu_core_id as usize);
    cpu_intc_subsystem.add_cpu_intc(cpu_intc1, *cpu_core_id1 as usize);
    cpu_intc_subsystem.add_cpu_intc(cpu_intc2, *cpu_core_id2 as usize);
    // Check CPU intc subsystem size
    if cpu_intc_subsystem.get_cpu_intc_array_size() > CPU_INTC_MAX_SIZE {
        panic!("CPU interrupt-controller sub-system should not exceed max size.");
    }

    // Check getting CPU intc
    let get_cpu_intc = cpu_intc_subsystem.get_cpu_intc(0).unwrap();
    if get_cpu_intc.get_cpu_intc_core_id() != 0 {
        panic!("CPU interrupt-controller sub-system return the wrong CPU interrupt-controller");
    }
    0
}

pub fn cpu_intc_subsystem_test_suite() {
    const CPU_INTC_TEST_SUITE: TestSuite = TestSuite {
        tests: &[
            TestCase::init(
                "CPU interrupt-controller sub-system basic implementation",
                test_cpu_intc_subsystem_impl,
                TestBehavior::Default,
            ),
            TestCase::init(
                "CPU interrupt-controller sub-system add same CPU interrupt-controller",
                test_cpu_intc_subsystem_same_device,
                TestBehavior::Default,
            ),
            TestCase::init(
                "CPU interrupt-controller sub-system handling overflow",
                test_cpu_intc_subsystem_overflow,
                TestBehavior::Default,
            ),
        ],
        name: "CPU interrupt-controller",
        behavior: TestSuiteBehavior::Default,
    };
    #[allow(static_mut_refs)]
    unsafe {
        TEST_MANAGER.add_suite(&CPU_INTC_TEST_SUITE)
    };
}
