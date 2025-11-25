#![no_std]
#![no_main]

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

use arch::traps::enable_interrupts;
use drivers::{cpufreq::CpuFreq, init_devices, timer::clint0::set_mtimecmp_ms};
use fdt::parse_dtb_file;

// Actually used in macro
#[allow(unused)]
use logs::LogLevel;

pub fn main(dtb_addr: usize) -> ! {
    parse_dtb_file(dtb_addr);
    init_devices();
    log!(LogLevel::Info, "LrnRTOS booting...");
    CpuFreq::init();
    set_mtimecmp_ms(20_000_000);
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
    loop {}
}
