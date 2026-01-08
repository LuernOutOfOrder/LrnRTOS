use crate::{
    drivers::DriverRegion,
    misc::RawTraitObject,
    platform::{self, DeviceType, InterruptExtended, platform_get_device_info},
    tests::TestCase,
};

use super::{TimerDevice, TimerSubSystem, TimerType, clint0::Clint0};

pub fn test_timer_subsystem_impl() {
    let timer_subsystem = TimerSubSystem::init();
    if timer_subsystem.get_timer_array_size() != 0 {
        panic!("Timer sub-system should be initialized empty.")
    }
    // Add timer to sub-system
    // Just unwrap, being in test env we know that it will return Some.
    let device_info = platform_get_device_info("sifive,clint0", DeviceType::Timer).unwrap();
    // Get struct behind trait
    let device_info_trait = device_info.info.unwrap();
    let raw: RawTraitObject = unsafe { core::mem::transmute(device_info_trait) };
    let timer_device_ptr = raw.data as *const platform::PlatformTimerDevice;
    let timer_device_ref = unsafe { &*timer_device_ptr };
    // Init timer driver
    let clint0: Clint0 = Clint0 {
        region: device_info.header.device_addr,
        interrupt_extended: timer_device_ref.interrupt_extended,
    };
    let device: TimerDevice = TimerDevice {
        timer_type: TimerType::ArchitecturalTimer,
        device: super::TimerDeviceDriver::Clint0(clint0),
    };
    timer_subsystem.add_timer(device);
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
    if primary_timer != device {
        panic!("Timer sub-system doesn't have the correct primary timer. Selection is wrong.");
    }
}

pub fn test_timer_subsystem_same_device() {
    let timer_subsystem = TimerSubSystem::init();
    // Add timer to sub-system
    // Just unwrap, being in test env we know that it will return Some.
    let device_info = platform_get_device_info("sifive,clint0", DeviceType::Timer).unwrap();
    // Get struct behind trait
    let device_info_trait = device_info.info.unwrap();
    let raw: RawTraitObject = unsafe { core::mem::transmute(device_info_trait) };
    let timer_device_ptr = raw.data as *const platform::PlatformTimerDevice;
    let timer_device_ref = unsafe { &*timer_device_ptr };
    // Init timer driver
    let clint0: Clint0 = Clint0 {
        region: device_info.header.device_addr,
        interrupt_extended: timer_device_ref.interrupt_extended,
    };
    let device: TimerDevice = TimerDevice {
        timer_type: TimerType::ArchitecturalTimer,
        device: super::TimerDeviceDriver::Clint0(clint0),
    };
    timer_subsystem.add_timer(device);
    // This should trigger a warning and abort timer registration
    timer_subsystem.add_timer(device);
    // Check if timer sub-system timer array has been updated
    if timer_subsystem.get_timer_array_size() > 1 {
        panic!("Timer sub-system should contain only 1 timer.")
    }
}

pub fn test_timer_subsystem_overflow() {
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
        device: super::TimerDeviceDriver::Clint0(clint0),
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
        device: super::TimerDeviceDriver::Clint0(clint1),
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
        device: super::TimerDeviceDriver::Clint0(clint2),
    };
    // Register all devices
    timer_subsystem.add_timer(device);
    timer_subsystem.add_timer(device1);
    #[allow(clippy::clone_on_copy)]
    let timer_subsystem_snapshot = unsafe { (*timer_subsystem.timer_pool.get()).clone() };
    // This one should trigger a warning and not be registered to the sub-system
    timer_subsystem.add_timer(device2);
    // Check if the subsystem has changed after the overflow aborted
    #[allow(clippy::clone_on_copy)]
    if timer_subsystem_snapshot != unsafe { (*timer_subsystem.timer_pool.get()).clone() } {
        panic!(
            "Timer sub-system state has changed after handling the overflow. This should not happened"
        );
    }
}

pub fn test_timer_subsystem_primary_timer() {
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
        device: super::TimerDeviceDriver::Clint0(clint0),
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
        device: super::TimerDeviceDriver::Clint0(clint1),
    };
    // Register all devices
    timer_subsystem.add_timer(device);
    timer_subsystem.add_timer(device1);
    // Check primary timer
    timer_subsystem.select_primary_timer();
    let primary_timer = timer_subsystem.get_primary_timer();
    if primary_timer != device1 {
        panic!("Timer sub-system primary timer should be the first ArchitecturalTimer registered");
    }
}

pub static TIMER_SUBSYSTEM_TEST_SUITE: &[TestCase] = &[
    TestCase {
        name: "Timer sub-system basic implementation",
        func: test_timer_subsystem_impl,
    },
    TestCase {
        name: "Timer sub-system add same device",
        func: test_timer_subsystem_same_device,
    },
    TestCase {
        name: "Timer sub-system handling overflow",
        func: test_timer_subsystem_overflow,
    },
    TestCase {
        name: "Timer sub-system check primary timer",
        func: test_timer_subsystem_primary_timer,
    },
];
