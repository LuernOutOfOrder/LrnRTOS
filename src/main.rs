#![no_std]
#![no_main]
#![warn(static_mut_refs)]

// Arch specific module
pub mod arch;
pub mod print;
// Devices module
pub mod devices;

use core::panic::PanicInfo;

pub fn main(dtb: u32) -> ! {
    devices::serials::ns16550::Ns16550::init();
    print::print("Hello");
    loop {
        unsafe {
            arch::interrupt::enable_and_halt();
        }
    }
}

#[panic_handler]
fn panic_handler(panic: &PanicInfo) -> ! {
    print::print(panic.message().as_str().unwrap());
    loop {
        unsafe {
            arch::interrupt::enable_and_halt();
        }
    }
}
