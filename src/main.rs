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
pub mod print;
// Module for kernel time
pub mod ktime;

use core::panic::PanicInfo;

use arch::traps::enable_interrupts;
use devices::{cpufreq::CpuFreq, init_devices};
use ktime::{ktime_seconds, set_ktime_ms};

pub fn main(dtb_addr: usize) -> ! {
    dtb::parse_dtb_file(dtb_addr);
    init_devices();
    print!("LrnRTOS booting...\n");
    CpuFreq::init();
    enable_interrupts();
    print!("Hello from LrnRTOS!\n");
    loop {
        let time = ktime_seconds();
        print!("interrupt timer working: {:?}\n", time);
        set_ktime_ms(20_000_000);
        unsafe {
            arch::traps::interrupt::enable_and_halt();
        }
    }
}

#[panic_handler]
fn panic_handler(panic: &PanicInfo) -> ! {
    print!("PANIC {:?}", panic);
    loop {
        unsafe {
            arch::traps::interrupt::enable_and_halt();
        }
    }
}
