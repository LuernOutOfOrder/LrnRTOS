use crate::{
    drivers::{
        DriverRegion,
        timer::{
            TimerDevice, TimerDeviceDriver, TimerSubSystem, TimerType, clint0::Clint0,
            init_timer_subsystem,
        },
    },
    platform::InterruptExtended,
    tests::{TEST_MANAGER, TestBehavior, TestCase, TestSuite},
};

pub fn test_timer_subsystem_impl() -> u8 {
    let timer_subsystem = TimerSubSystem::init();
    if timer_subsystem.get_timer_array_size() != 0 {
        panic!("Timer sub-system should be initialized empty.")
    }
    // Add timer to sub-system
    // Init timer driver
    const CLINT0: Clint0 = Clint0 {
        region: DriverRegion {
            addr: 0x2000000,
            size: 0x10000,
        },
        interrupt_extended: [InterruptExtended {
            cpu_intc: 0,
            irq_len: 2,
            irq_ids: [3, 7, 0, 0],
        }; 4],
    };
    const DEVICE: TimerDevice = TimerDevice {
        timer_type: TimerType::ArchitecturalTimer,
        device: TimerDeviceDriver::Clint0(CLINT0),
    };
    timer_subsystem.add_timer(DEVICE);
    // Check if timer sub-system timer array has been updated
    if timer_subsystem.get_timer_array_size() != 1 {
        panic!("Timer sub-system should not be empty.")
    }

    // Select primary timer
    timer_subsystem.select_primary_timer();
    // Check if timer sub-system timer array has been updated
    if timer_subsystem.get_timer_array_size() != 0 {
        panic!("Timer sub-system should be empty.")
    }
    // Check primary timer
    let primary_timer = timer_subsystem.get_primary_timer();
    if primary_timer != &DEVICE {
        panic!("Timer sub-system doesn't have the correct primary timer. Selection is wrong.");
    }

    // Initialize timer subsystem, don't know where to put it instead of here
    init_timer_subsystem();
    0
}

pub fn test_timer_subsystem_same_device() -> u8 {
    let timer_subsystem = TimerSubSystem::init();
    // Add timer to sub-system
    // Init timer driver
    const CLINT0: Clint0 = Clint0 {
        region: DriverRegion {
            addr: 0x2000000,
            size: 0x10000,
        },
        interrupt_extended: [InterruptExtended {
            cpu_intc: 0,
            irq_len: 2,
            irq_ids: [3, 7, 0, 0],
        }; 4],
    };
    const DEVICE: TimerDevice = TimerDevice {
        timer_type: TimerType::ArchitecturalTimer,
        device: TimerDeviceDriver::Clint0(CLINT0),
    };
    timer_subsystem.add_timer(DEVICE);
    // This should trigger a warning and abort timer registration
    timer_subsystem.add_timer(DEVICE);
    // Check if timer sub-system timer array has been updated
    if timer_subsystem.get_timer_array_size() > 1 {
        panic!("Timer sub-system should contain only 1 timer.")
    }
    0
}

pub fn test_timer_subsystem_overflow() -> u8 {
    let timer_subsystem = TimerSubSystem::init();
    // Build multiple timer to test how the subsystem handle overflow
    let int_ext = [InterruptExtended {
        cpu_intc: 0,
        irq_len: 2,
        irq_ids: [3, 7, 0, 0],
    }; 4];
    // First timer
    let clint0: Clint0 = Clint0 {
        region: DriverRegion {
            addr: 0x2000000,
            size: 0x10000,
        },
        interrupt_extended: int_ext,
    };
    let device: TimerDevice = TimerDevice {
        timer_type: TimerType::SoCTimer,
        device: TimerDeviceDriver::Clint0(clint0),
    };
    // Second timer
    let clint1: Clint0 = Clint0 {
        region: DriverRegion {
            addr: 0x2000001,
            size: 0x10001,
        },
        interrupt_extended: int_ext,
    };
    let device1: TimerDevice = TimerDevice {
        timer_type: TimerType::ArchitecturalTimer,
        device: TimerDeviceDriver::Clint0(clint1),
    };
    // Third timer
    let clint2: Clint0 = Clint0 {
        region: DriverRegion {
            addr: 0x2000002,
            size: 0x10002,
        },
        interrupt_extended: int_ext,
    };
    let device2: TimerDevice = TimerDevice {
        timer_type: TimerType::ArchitecturalTimer,
        device: TimerDeviceDriver::Clint0(clint2),
    };
    // Register all devices
    timer_subsystem.add_timer(device);
    timer_subsystem.add_timer(device1);
    let timer_subsystem_snapshot = unsafe {
        [
            &*timer_subsystem.timer_pool[0].get(),
            &*timer_subsystem.timer_pool[1].get(),
        ]
    };
    // This one should trigger a warning and not be registered to the sub-system
    timer_subsystem.add_timer(device2);
    // Recreate a snapshot of subsystem
    let timer_subsystem_snapshot_updated = unsafe {
        [
            &*timer_subsystem.timer_pool[0].get(),
            &*timer_subsystem.timer_pool[1].get(),
        ]
    };
    // Check if the subsystem has changed after the overflow aborted
    if timer_subsystem_snapshot != timer_subsystem_snapshot_updated {
        panic!(
            "Timer sub-system state has changed after handling the overflow. This should not happened"
        );
    }
    0
}

pub fn test_timer_subsystem_primary_timer() -> u8 {
    let timer_subsystem = TimerSubSystem::init();
    // Build multiple timer to test how the subsystem handle overflow
    const INT_EXT: [InterruptExtended; 4] = [InterruptExtended {
        cpu_intc: 0,
        irq_len: 2,
        irq_ids: [3, 7, 0, 0],
    }; 4];
    // First timer
    let clint0: Clint0 = Clint0 {
        region: DriverRegion {
            addr: 0x2000000,
            size: 0x10000,
        },
        interrupt_extended: INT_EXT,
    };
    let device: TimerDevice = TimerDevice {
        timer_type: TimerType::SoCTimer,
        device: TimerDeviceDriver::Clint0(clint0),
    };
    // Second timer
    const CLINT1: Clint0 = Clint0 {
        region: DriverRegion {
            addr: 0x2000001,
            size: 0x10001,
        },
        interrupt_extended: INT_EXT,
    };
    const DEVICE1: TimerDevice = TimerDevice {
        timer_type: TimerType::ArchitecturalTimer,
        device: TimerDeviceDriver::Clint0(CLINT1),
    };
    // Register all devices
    timer_subsystem.add_timer(device);
    timer_subsystem.add_timer(DEVICE1);
    // Check primary timer
    timer_subsystem.select_primary_timer();
    let primary_timer = timer_subsystem.get_primary_timer();
    if *primary_timer != DEVICE1 {
        panic!("Timer sub-system primary timer should be the first ArchitecturalTimer registered");
    }
    0
}

pub fn timer_subsystem_test_suite() {
    const TIMER_SUB_SYSTEM_TEST_SUITE: TestSuite = TestSuite {
        tests: &[
            TestCase::init(
                "Timer sub-system basic implementation",
                test_timer_subsystem_impl,
                TestBehavior::Default,
            ),
            TestCase::init(
                "Timer sub-system add same device",
                test_timer_subsystem_same_device,
                TestBehavior::Default,
            ),
            TestCase::init(
                "Timer sub-system handling overflow",
                test_timer_subsystem_overflow,
                TestBehavior::Default,
            ),
            TestCase::init(
                "Timer sub-system check primary timer",
                test_timer_subsystem_primary_timer,
                TestBehavior::Default,
            ),
        ],
        name: "Timer sub-system",
        tests_nb: 4,
    };

    #[allow(static_mut_refs)]
    unsafe {
        TEST_MANAGER.add_suite(&TIMER_SUB_SYSTEM_TEST_SUITE)
    };
}
