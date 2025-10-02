#![no_std]
#![no_main]
#![warn(static_mut_refs)]

// Arch specific module
pub mod arch;
pub mod print;
// Devices module
pub mod devices;
mod dtb;

use core::panic::PanicInfo;

pub fn main(dtb_addr: *const u8) -> ! {
    // dtb::parse_dtb_file(dtb_addr);
    devices::serials::ns16550::Ns16550::init();
    print!("Hello from LrnRTOS!");
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
