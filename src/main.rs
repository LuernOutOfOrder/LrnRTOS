#![no_std]
#![no_main]

// Arch specific module
pub mod arch;
pub mod print;
// Devices module
pub mod devices;

use core::panic::PanicInfo;

use devices::serials::{ns16550::Ns16550, UartDriver};

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
    loop {}
}
