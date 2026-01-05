// See documentation in `Documentation/hardware/soc/riscv/clint.md`

use core::ptr::{self};

use crate::{
    drivers::DriverRegion,
    misc::RawTraitObject,
    platform::{self, DeviceType, InterruptExtended, platform_get_device_info},
};

use super::{TIMER_SUBSYSTEM, Timer, TimerDevice, TimerType};

#[derive(Copy, Clone)]
pub struct Clint0 {
    region: DriverRegion,
    #[allow(unused)]
    interrupt_extended: [InterruptExtended; 4],
}

impl Timer for Clint0 {
    fn read_time(&self) -> u64 {
        self.read_mtime()
    }

    fn set_delay(&self, core: usize, delay: u64) {
        self.set_mtimecmp(core, delay);
    }
}

impl Clint0 {
    pub fn init() {
        let device_info = match platform_get_device_info("sifive,clint0", DeviceType::Timer) {
            Some(d) => d,
            None => return,
        };
        // Get struct behind trait
        let device_info_trait = device_info.info.unwrap();
        let raw: RawTraitObject = unsafe { core::mem::transmute(device_info_trait) };
        let timer_device_ptr = raw.data as *const platform::TimerDevice;
        let timer_device_ref = unsafe { &*timer_device_ptr };
        // Init Clint0 driver and update timer sub-system for global access.
        let clint0: Clint0 = Clint0 {
            region: device_info.header.device_addr,
            interrupt_extended: timer_device_ref.interrupt_extended,
        };
        let device: TimerDevice = TimerDevice {
            timer_type: TimerType::ArchitecturalTimer,
            device: super::TimerDeviceDriver::Clint0(clint0),
        };
        TIMER_SUBSYSTEM.add_timer(device);
    }

    /// Read mtime from clint0 addr + offset from `https://chromitem-soc.readthedocs.io/en/latest/clint.html`
    /// Check 2 time value from high addr to avoid miscompute mtime and giving wrong tick, and led
    /// to UB.
    pub fn read_mtime(&self) -> u64 {
        // Offset from doc
        let off = 0xBFF8;
        // Define mtime value
        let mut mtime_low: u32 = 0;
        let mut mtime_high: u32 = 0;
        // Define mtime_high checking value to make the while loop work
        let mut mtime_high_check: u32 = 1;

        // While the two mtime is different continue to read to avoid miss compute mtime and lead
        // to UB.
        while mtime_high != mtime_high_check {
            let mtime_low_addr = self.region.addr + off;
            let mtime_high_addr = self.region.addr + off + 4;
            mtime_high = unsafe { ptr::read_volatile(mtime_high_addr as *const u32) };
            mtime_low = unsafe { ptr::read_volatile(mtime_low_addr as *const u32) };
            mtime_high_check = unsafe { ptr::read_volatile(mtime_high_addr as *const u32) };
        }
        // Bitwise to compute mtime from value. Cannot read u64 directly on RISC-V 32 bits.
        let output: u64 = ((mtime_high as u64) << 32) | (mtime_low as u64);
        output
    }

    /// Set a timer
    /// hart_id: id of the target hart to send timer interrupt.
    /// next_tick: value to set the timer to, current-time + next_tick
    pub fn set_mtimecmp(&self, hart_id: usize, next_tick: u64) {
        let off = 0x4000 + (hart_id * 8);
        // Value to deactivate temporaly interrupt
        let deactivate_int: u32 = 0xFFFF_FFFF;
        let mtimecmp_low_addr = self.region.addr + off;
        let mtimecmp_high_addr = self.region.addr + off + 4;
        let next_tick_high: u32 = (next_tick >> 32) as u32;
        let next_tick_low: u32 = (next_tick & 0xFFFF_FFFF) as u32;
        unsafe {
            // First write deactivate_int value to temporaly deactivate interrupt from hardware
            ptr::write_volatile(mtimecmp_high_addr as *mut u32, deactivate_int);
            // Second write next_tick_low value to low addr of the set_mtimecmp addr
            ptr::write_volatile(mtimecmp_low_addr as *mut u32, next_tick_low);
            // And finally write the next_tick_high value to high addr of the set_mtimecmp addr
            ptr::write_volatile(mtimecmp_high_addr as *mut u32, next_tick_high);
        }
    }

    /// Send an interrupt to given hart by writting 1 to the clint0 addr + hart_id * 4.
    /// hart_id: id of the target hart to send interrupt.
    pub fn send_ipi(&self, hart_id: usize) {
        let addr = self.region.addr + (hart_id * 4);
        unsafe { ptr::write_volatile(addr as *mut u32, 1) };
    }
}
