#![no_std]
#![no_main]
#![warn(static_mut_refs)]

// Arch specific module
pub mod arch;
// Devices module
pub mod devices;
// Dtb module
mod dtb;
// Logging modules
pub mod log;
pub mod print;

use core::panic::PanicInfo;

use dtb::get_all_fdt_nodes_name;

pub fn main(dtb_addr: usize) -> ! {
    dtb::parse_dtb_file(dtb_addr);
    get_all_fdt_nodes_name();
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
    print!("{:?}", panic);
    loop {
        unsafe {
            arch::interrupt::enable_and_halt();
        }
    }
}
