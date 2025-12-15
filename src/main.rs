#![no_std]
#![no_main]

// Config module
pub mod config;

// Arch specific module
pub mod arch;

// Drivers module
pub mod drivers;

// Device init
pub mod devices_info;
pub mod platform;

// Logging modules
pub mod kprint;
pub mod logs;
pub mod print;

// Module for kernel time
pub mod ktime;

// Memory management module
pub mod mem;

// Misc mod
pub mod misc;

use core::panic::PanicInfo;

// Use from modules
use arch::traps::{enable_interrupts, trap_frame::init_trap_frame};
use config::TICK_SAFETY_DURATION;
use drivers::{cpufreq::CpuFreq, init_devices_subsystems};
use ktime::set_ktime_seconds;
use logs::LogLevel;
use mem::memory_init;
use platform::platform_init;

#[unsafe(no_mangle)]
pub fn main(core: usize, dtb_addr: usize) -> ! {
    kprint_fmt!("Start kernel booting on CPU Core: {}.\n", core);
    kprint!("Initializing platform...");
    platform_init(dtb_addr);
    kprint!("Initializing all sub-systems...\n");
    init_devices_subsystems();
    log!(LogLevel::Info, "Successfully initialized all sub-system.");
    log!(LogLevel::Info, "Initializing memory...");
    memory_init();
    log!(LogLevel::Info, "Successfully initialized memory.");
    log!(LogLevel::Info, "LrnRTOS booting...");
    CpuFreq::init();
    log!(LogLevel::Debug, "Initialing trap frame...");
    init_trap_frame();
    log!(LogLevel::Debug, "Successfully initialized trap frame.");
    set_ktime_seconds(TICK_SAFETY_DURATION);
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
