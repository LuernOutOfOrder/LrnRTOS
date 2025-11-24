#![no_std]
#![no_main]

// Arch specific module
pub mod arch;

// Devices module
pub mod devices;

// Device tree module
mod dtb;

// Logging modules
pub mod log;
pub mod logs;
pub mod print;

// Module for kernel time
pub mod ktime;

use core::panic::PanicInfo;

use arch::traps::enable_interrupts;
use devices::{cpufreq::CpuFreq, init_devices};
use ktime::set_ktime_tick_safety;

// Actually used in macro
#[allow(unused)]
use logs::LogLevel;

pub fn main(dtb_addr: usize) -> ! {
    dtb::parse_dtb_file(dtb_addr);
    init_devices();
    log!(LogLevel::Info, "LrnRTOS booting...");
    CpuFreq::init();
    set_ktime_tick_safety(20_000_000);
    enable_interrupts();
    log!(LogLevel::Info, "LrnRTOS started!");
    loop {
        unsafe {
            arch::traps::interrupt::enable_and_halt();
        }
    }
}

#[panic_handler]
fn panic_handler(panic: &PanicInfo) -> ! {
    print!("PANIC {:?}", panic);
    loop {
    }
}
