#![no_std]
#![no_main]

// Config module
pub mod config;

// Arch specific module
pub mod arch;

// Drivers module
pub mod drivers;

// Device tree module
mod fdt;

// Logging modules
pub mod kprint;
pub mod logs;
pub mod print;

// Module for kernel time
pub mod ktime;

use core::panic::PanicInfo;

use arch::traps::{enable_interrupts, trap_frame::init_trap_frame};
use drivers::{cpufreq::CpuFreq, init_devices_subsystems};
use fdt::parse_dtb_file;

use ktime::set_ktime_seconds;
use logs::LogLevel;

#[unsafe(no_mangle)]
pub fn main(dtb_addr: usize) -> ! {
    parse_dtb_file(dtb_addr);
    kprint!("Initializing all sub-systems...\n");
    init_devices_subsystems();
    log!(LogLevel::Info, "Successfully initialized all sub-system.");
    log!(LogLevel::Info, "LrnRTOS booting...");
    CpuFreq::init();
    log!(LogLevel::Debug, "Initialing trap frame...");
    init_trap_frame();
    log!(LogLevel::Debug, "Successfully initialized trap frame.");
    set_ktime_seconds(1);
    enable_interrupts();
    log!(LogLevel::Info, "LrnRTOS started!");
    loop {
        log!(LogLevel::Debug, "Main loop uptime.");
        unsafe {
            arch::traps::interrupt::enable_and_halt();
        }
    }
}

#[panic_handler]
fn panic_handler(panic: &PanicInfo) -> ! {
    kprint_fmt!("PANIC {:?}", panic);
    loop {}
}
