#![no_std]
#![no_main]

pub mod arch;
pub mod print;

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
