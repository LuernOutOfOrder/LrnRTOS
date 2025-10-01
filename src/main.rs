#![no_std]
#![no_main]

// Arch specific module
pub mod arch;
pub mod print;
// Devices module
pub mod devices;

use core::panic::PanicInfo;

fn main() -> ! {
    print::print("Hello world!");
    loop {
        unsafe {
            arch::interrupt::enable_and_halt();
        }
    }
}

#[panic_handler]
fn panic_handler(panic: &PanicInfo) -> ! {
    loop {}
}
