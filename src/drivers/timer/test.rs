use crate::{
    misc::RawTraitObject, platform::{self, platform_get_device_info, DeviceType}, tests::TestCase
};

use super::{clint0::Clint0, TimerDevice, TimerSubSystem, TimerType};

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
    let timer_device_ptr = raw.data as *const platform::TimerDevice;
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

pub static TIMER_SUBSYSTEM_TEST_SUITE: &[TestCase] = &[TestCase {
    name: "Timer sub-system basic implementation",
    func: test_timer_subsystem_impl,
}];
